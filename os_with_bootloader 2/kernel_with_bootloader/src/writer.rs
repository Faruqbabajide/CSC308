mod constants;

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use constants::font_constants;
use constants::font_constants::{BACKUP_CHAR, CHAR_RASTER_HEIGHT, FONT_WEIGHT};
use core::{
    fmt::{self, Write},
    ptr,
};
use noto_sans_mono_bitmap::{RasterizedChar, get_raster};

/// Additional vertical space between lines
const LINE_SPACING: usize = 2;

/// Additional horizontal space between characters
const LETTER_SPACING: usize = 0;

/// Padding from the border. Prevents the font from being too close to the border
const BORDER_PADDING: usize = 1;

/// Returns the raster of the given char or the raster of [font_constants::BACKUP_CHAR].
fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR.chars().next().unwrap()).expect("Should get raster of backup char."))
}


/// Allows logging text to a pixel-based framebuffer.
pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
    current_color: [u8; 4], // Current text color
    is_escape: bool,        // Tracks if we are in an escape sequence
    is_color_change: bool,  // Tracks if we are changing color
}

impl FrameBufferWriter {
    /// Creates a new logger that uses the given framebuffer.
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut logger = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
            current_color: [255, 255, 255, 255], // Default: White
            is_escape: false,       // Initialize escape tracking
            is_color_change: false, // Initialize color change tracking
        };
        logger.clear();
        logger
    }    

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        if x < self.width() && y < self.height() {
            self.x_pos = x;
            self.y_pos = y;
        } else {
            panic!("Cursor position out of bounds: ({}, {})", x, y);
        }
    }

    fn scroll(&mut self) {
        let row_height = font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        let stride_bytes = self.info.stride * self.info.bytes_per_pixel;
        let total_bytes = self.framebuffer.len();

        // Move rows up by one row height
        let visible_bytes = stride_bytes * (self.height() - row_height);
        self.framebuffer
            .copy_within(stride_bytes..stride_bytes + visible_bytes, 0);

        // Clear the last row (set to black)
        let clear_start = total_bytes - stride_bytes * row_height;
        self.framebuffer[clear_start..].fill(0);

        // Adjust cursor to the last visible row
        self.y_pos = self.height() - row_height;
        self.x_pos = BORDER_PADDING;
    }

    fn newline(&mut self) {
        let row_height = font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.y_pos += row_height;

        if self.y_pos >= self.height() {
            self.scroll();
        }
        self.carriage_return();
    }

    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING; // Reset the horizontal cursor position
    }

    /// Erases all text on the screen. Resets `self.x_pos` and `self.y_pos`.
    pub fn clear(&mut self) {
        self.x_pos = BORDER_PADDING;
        self.y_pos = BORDER_PADDING;
        self.framebuffer.fill(0);
    }

/// Writes a single char to the framebuffer. Takes care of special control characters and escape sequences.
fn write_char(&mut self, c: char) {
    match c {
        '\n' => self.newline(),
        '\r' => self.carriage_return(),
        '\t' => {
            self.x_pos += 4 * font_constants::CHAR_RASTER_HEIGHT.val(); // Adjust tab space
            if self.x_pos >= self.width() {
                self.newline();
            }
            self.reset_color(); // Reset color back to default (white) for indented text
        }
        '\\' => {  // Detect a backslash (start of escape sequence)
            self.is_escape = true;  // Enable escape mode
        }
        'c' if self.is_escape => {  // Detect '\\c' for blue color change
            self.is_escape = false;  // Reset escape mode
            self.current_color = [255, 0, 0, 0];  // Set color to blue (RGB: 0, 0, 255, Alpha: 255)
        }
        _ => {
            // Reset escape modes if no escape sequence is detected
            self.is_escape = false;
            self.is_color_change = false;

            let new_xpos = self.x_pos + font_constants::CHAR_RASTER_HEIGHT.val();
            if new_xpos >= self.width() {
                self.newline();
            }
            if self.y_pos + font_constants::CHAR_RASTER_HEIGHT.val() >= self.height() {
                self.scroll();
            }
            self.write_rendered_char(get_char_raster(c));  // Render the character with the current color
        }
    }
}

/// Reset the color to default (white).
fn reset_color(&mut self) {
    self.current_color = [255, 255, 255, 255];  // Default white color (RGBA: 255, 255, 255, 255)
}


    /// Prints a rendered char into the framebuffer.
    /// Updates `self.x_pos`.
    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }
        self.x_pos += rendered_char.width() + LETTER_SPACING;
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let bytes_per_pixel = self.info.bytes_per_pixel;
    
        // Ensure the blue color (RGBA) is set correctly
        let color = [
            (self.current_color[0] as u16 * intensity as u16 / 255) as u8, // Scale Red
            (self.current_color[1] as u16 * intensity as u16 / 255) as u8, // Scale Green
            (self.current_color[2] as u16 * intensity as u16 / 255) as u8, // Scale Blue
            255, // Alpha (fully opaque)
        ];
    
        // Apply the color based on the framebuffer's pixel format
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
    
        // Optionally read from framebuffer (for debugging or further optimization)
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }
}    
unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

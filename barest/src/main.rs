#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Write a simple message to the screen
    vga_buffer::print("Welcome to Faruq's Kernel!");

    // Infinite loop
    loop {}
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

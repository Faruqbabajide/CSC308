#include <stdio.h>

int main() {
    int choice;
    printf("Choose the data type:\n");
    printf("1. Character\n2. Integer\n3. Float\n");
    printf("Enter your choice (1/2/3): ");
    scanf("%d", &choice);

    switch (choice) {
        case 1: {
            char ch;
            printf("Enter a character: ");
            scanf(" %c", &ch);

            printf("\nNext four characters in multiples of 3:\n");
            for (int i = 1; i <= 4; i++) {
                char next_ch = ch + (i * 3);
                printf("Character: %c, ASCII Code: %d\n", next_ch, next_ch);
            }

            printf("Size of Character: %lu bytes\n", sizeof(ch));
            break;
        }
        case 2: {
            int num;
            printf("Enter an integer: ");
            scanf("%d", &num);

            printf("\nNext four integers in multiples of 3:\n");
            for (int i = 1; i <= 4; i++) {
                int next_num = num + (i * 3);
                printf("Integer: %d\n", next_num);
            }

            printf("Size of Integer: %lu bytes\n", sizeof(num));
            break;
        }
        case 3: {
            float num;
            printf("Enter a float: ");
            scanf("%f", &num);

            printf("\nNext four floats in multiples of 3:\n");
            for (int i = 1; i <= 4; i++) {
                float next_num = num + (i * 3.0);
                printf("Float: %.2f\n", next_num);
            }

            printf("Size of Float: %lu bytes\n", sizeof(num));
            break;
        }
        default:
            printf("Invalid choice.\n");
    }

    return 0;
}

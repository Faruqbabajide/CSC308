#include <stdio.h>

int main() {
    float a, b, result;
    char operation;

    printf("Enter a: ");
    scanf("%f", &a);

    printf("Enter b: ");
    scanf("%f", &b);



    printf("Choose an operation (+, -, *, /): ");
    scanf(" %c", &operation);

    
    switch (operation) {
        case '+':
            result = a + b;
            printf("Result (Sum): %f\n", result);
            break;

        case '-':
            result = a -b ;
            printf("Result (Subtraction): %f\n", result);
            break;

        case '*':
            result = a *b;
            printf("Result (Multiplication): %f\n", result);
            break;

        case '/':
        
            if (b != 0 && a != 0) {
                result = a/b;
                printf("Result (Division): %.2f\n" 
                , result);
            } else {
                printf("Error: Division by zero is not allowed.\n");
            }
            break;

        default:
            printf("Invalid operation. Please choose +, -, *, or /.\n");
    }

    return 0;
}

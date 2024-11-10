#include <stdio.h>

int main() {
    int age;
    char experience;

    // Input age
    printf("Enter age: ");
    scanf("%d", &age);

    // Input experience (Y for yes, N for no)
    printf("Do you have experience? (Y/N): ");
    scanf(" %c", &experience);

    // Check experience and age to determine salary
    if (experience == 'Y' || experience == 'y') {
        if (age >= 40) {
            printf("Salary: N560,000\n");
        } else if (age >= 30 && age < 40) {
            printf("Salary: N480,000\n");
        } else if (age < 28) {
            printf("Salary: N300,000\n");
        } else {
            printf("Salary: N300,000\n"); // Just in case age 28 or 29
        }
    } else {
        printf("Salary: N100,000\n");
    }

    return 0;
}

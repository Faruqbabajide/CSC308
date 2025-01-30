#include <stdio.h>

int main() {
    int rows = 1, cols = 3;

    int matrix[1][3] = {{1, 2, 3}};
    int matrix2[1][3] = {{1, 7, 3}};
    int result[1][3];

    printf("The resulting matrix is:\n");
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            result[i][j] = matrix[i][j] + matrix2[i][j]; 
            printf("%d ", result[i][j]);
        }
        printf("\n");
    }

    return 0;
}
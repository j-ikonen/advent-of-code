#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <stdlib.h>

#define MAX 160

int main() {
        int sum = 0;
        
        FILE *fptr;
        fptr = fopen("input", "r");
        if (!fptr) {
                printf("Input file could not be opened.");
                exit(0);
        }

        char line[MAX];
        while (fgets(line, MAX, fptr)) {

        }

        printf("Result:\n");
        printf("%i\n", sum);
        return 0;

        return 0;
}

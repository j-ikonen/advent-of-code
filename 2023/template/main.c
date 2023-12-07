#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <stdlib.h>

#define MAX 160

int main() {
        FILE *fptr;
        fptr = fopen("input", "r");
        if (!fptr) {
                printf("Input file could not be opened.");
                exit(0);
        }

        char line[MAX];
        while (fgets(line, MAX, fptr)) {

        }

        return 0;
}

#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>

#define MAX 160


int main() {
        FILE *fptr;
        fptr = fopen("input", "r");
        if (!fptr) {
                printf("Input file could not be opened.");
                exit(0);
        }

        char line[MAX];
        char num[3];
        num[2]='\0';
        char *cur = line;
        char first = -1;
        char last = -1;
        int sum = 0;
        int add = 0;
        const char *digits[] = {
                "one\0",
                "two\0",
                "three\0",
                "four\0",
                "five\0",
                "six\0",
                "seven\0",
                "eight\0",
                "nine\0"
        };
        while (fgets(line, MAX, fptr)) {
                cur = line;
                // printf("Line: %s\n", line);
                while (*cur != '\0') {
                        // Check if char is digit
                        if (isdigit(*cur)) {
                                if (first == -1) {
                                        first = *cur;
                                }
                                last = *cur;

                        // Check for string digit
                        } else {
                                for (int i = 0; i < 9; i++) {
                                        if (!strncmp(cur, digits[i], strlen(digits[i]))) {
                                                if (first == -1) {
                                                        first = i+1 + '0';
                                                }
                                                last = i+1 + '0';
                                        }
                                }
                        }
                        cur++;
                }
                // printf("first: %d\n", first - '0');
                // printf("last: %d\n", last - '0');
                num[0] = first;
                num[1] = last;
                add = atoi(num);
                // printf("num: %s\n", num);
                // printf("add: %d\n", add);
                sum += add;
                first = -1;
                last = -1;
        }
        printf("Resut:\n");
        printf("%i\n", sum);
        return 0;
}

#define _CRT_SECURE_NO_WARNINGS
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX 142
#define LINES 140

struct gear {
        int row;
        int col;
        int n;
        int num[4];
};

int main() {
        int sum = 0;
        char *schema[LINES];
        struct gear gears[1000] = {{0, 0, 0, {0}}};
        int gear_end = 0;
        
        FILE *fptr;
        fptr = fopen("input", "r");
        if (!fptr) {
                printf("Input file could not be opened.");
                exit(0);
        }

        char line[MAX];
        int i = 0;
        int lines_alloc = 0;
        while (1) {
                char * ptr = malloc(MAX);
                lines_alloc++;
                
                if (!fgets(ptr, MAX, fptr)) {
                        free(ptr);
                        // printf("Lines allocated: %i\n", lines_alloc-1);
                        break;
                }
                schema[i] = ptr;
                i++;
        }

        // for (int i = 0; i < LINES; i++) {
        //         printf("%s",schema[i]);
        // }

        int start = -1;
        int end = -1;
        int rs = -1;
        int re = -1;
        int still_n = 0;
        int is_pn = 0;
        char number[10];
        int part_count = 0;

        for (int row = 0; row < LINES; row++) {
                for (int col = 0; col < MAX-1; col++) {
                        
                        // Found end of number
                        if (!isdigit(schema[row][col]) && still_n) {
                                if (col == MAX-2) end = MAX-2;
                                else end = col+1;
                                if (row == 0) rs = 0;
                                else rs = row - 1;
                                if (row+1 == LINES) re = LINES-1;
                                else re = row + 1;
                                still_n = 0;

                                // printf("Explore [%i,%i], [%i,%i]\n", rs, start, re, end-1);
                                // Explore adjacent for a symbol to determine if this is a part number
                                for (int nrow = rs; nrow <= re; nrow++) {
                                        for (int ncol = start; ncol < end; ncol++) {
                                                // printf("%c", schema[nrow][ncol]);
                                                if (!isdigit(schema[nrow][ncol]) && !(schema[nrow][ncol] == '.')) {
                                                        is_pn = 1;
                                                        if (!isdigit(schema[row][start])) {start = start + 1; }
                                                        if (!isdigit(schema[row][end])) {end = end - 1; }
                                                        strncpy(number, &schema[row][start], 4);
                                                        number[end] = '\0';
                                                        // printf("Old sum: %i", sum);
                                                        int temp = atoi(number);
                                                        sum += temp;
                                                        part_count++;
                                                        // Add possible gear
                                                        if (schema[nrow][ncol]=='*') {
                                                                int added = 0;
                                                                for (int i = 0; i < gear_end; i++) {
                                                                        if (gears[i].col == ncol && gears[i].row == nrow ) {
                                                                                gears[i].num[gears[i].n] = temp;
                                                                                gears[i].n++;
                                                                                added = 1;
                                                                        }
                                                                }
                                                                if (!added) {
                                                                        struct gear g = {nrow, ncol, 1, {0}};
                                                                        g.num[0] = temp;
                                                                        gears[gear_end] = g;
                                                                        gear_end++;
                                                                }
                                                        }
                                                        // printf("New sum: %i", sum);
                                                        // printf("\nPart number: %s by symbol: '%c'\n", number, schema[nrow][ncol]);
                                                }
                                        }
                                        // printf("\n");
                                        if (is_pn) {
                                                is_pn=0;
                                                break;
                                        }
                                }

                        // Found start of number
                        } else if (isdigit(schema[row][col]) && !still_n) {
                                if (col == 0)   start = col;
                                else            start = col-1;
                                still_n = 1;
                                // printf("\nNumber at [%i, %i]\n", row, col);
                        }

                }
        }
        int gearsum = 0;
        for (int i = 0; i < gear_end; i++) {
                if (gears[i].n == 2) {
                        gearsum += gears[i].num[0] * gears[i].num[1];
                }
        }

        printf("Result:\n");
        printf("%i\n", sum);
        printf("Parts:\n");
        printf("%i\n", part_count);
        printf("Gearsum:\n");
        printf("%i\n", gearsum);

        for (int i = 0; i < LINES; i++) { free(schema[i]); }
        return 0;
}

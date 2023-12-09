#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX 256
#define MAX_RED 12
#define MAX_GREEN 13
#define MAX_BLUE 14
const char sred[]= "red";
const char sgreen[] = "green";
const char sblue[5] = "blue\0";

int main() {
        FILE *fptr;
        fptr = fopen("input", "r");
        if (!fptr) {
                printf("Input file could not be opened.");
                exit(0);
        }

        int max_r = 0;
        int max_g = 0;
        int max_b = 0;
        int sum_id = 0;
        int cur_id = 0;
        int cur_val = 0;
        int impossible = 0;
        int power = 0;
        char *token;
        const char delim[5] = " ,;:";

        char line[MAX];
        while (fgets(line, MAX, fptr)) {
                token = strtok(line, delim);    // "Game"
                token = strtok(NULL, delim);    // "ID"
                cur_id = atoi(token);
                while (1) {
                        token = strtok(NULL, delim);    // "VALUE"
                        if (token == NULL) break;
                        cur_val = atoi(token);
                        token = strtok(NULL, delim);    // "COLOR"
                        if (strstr(token, sred) != NULL && max_r < cur_val) {
                                max_r = cur_val;
                                // if (cur_val > MAX_RED) { impossible = 1; }
                        } else if (strstr(token, sgreen) != NULL && max_g < cur_val) {
                                max_g = cur_val;
                                // if (cur_val > MAX_GREEN) { impossible = 1; }
                        } else if (strstr(token, sblue) != NULL && max_b < cur_val) {
                                max_b = cur_val;
                                // if (cur_val > MAX_BLUE) { impossible = 1; }
                        } 
                        // else {
                        //         printf("Invalid color token: '%s'\n", token);
                        // }
                }
                // if (!impossible) {
                //         sum_id += cur_id;
                // }
                // impossible = 0;

                sum_id += max_r * max_g * max_b;
                

                // printf("Game %i: r:%i g:%i b:%i", cur_id, max_r, max_g, max_b);
                // // printf("Testing Game %i\n", cur_id);
                // if (max_r > MAX_RED || max_g > MAX_GREEN || max_b > MAX_BLUE) {
                //         printf(" IMPOSSIBLE\n");
                // } else {
                //         sum_id += cur_id;
                //         printf("\n");
                // }
                max_r = 0;
                max_g = 0;
                max_b = 0;
        }

        printf("Resut:\n");
        printf("%i\n", sum_id);
        return 0;

        return 0;
}

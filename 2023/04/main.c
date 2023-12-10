#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX 118
#define NWIN 10
#define NMY 25
#define LINES 220


void collect(int *buf, char *ptr, int n, const char *delim) {
        char *token = strtok(ptr, delim);
        for (int i = 0; i < n; i++) {
                if (!token) break;
                buf[i] = atoi(token);
                token = strtok(NULL, delim);
        }
}

typedef int find_t;
find_t *find(find_t *buf, int value, int sz) {
        find_t *ptr = NULL;
        for (int i = 0; i < sz; i++) {
                if (buf[i] == value) {
                        ptr = &buf[i];
                }
        }
        return ptr;   
}


int main() {
        int sum = 0;
        int line_score = 0;
        
        FILE *fptr;
        fptr = fopen("input", "r");
        if (!fptr) {
                printf("Input file could not be opened.");
                exit(0);
        }

        char line[MAX];
        char *tok, *win, *myn;
        int winners[NWIN], mynum[NMY];
        int cards[LINES] = {1};
        for (int i = 0; i < LINES; i++) {
                cards[i] = 1;
        }
        int nc = 0, win_offset = 1;

        while (fgets(line, MAX, fptr)) {
                tok = strtok(line, ":");
                tok = strtok(NULL, ":");
                win = strtok(tok, "|");
                myn = strtok(NULL, "|");

                // Collect winning and my numbers
                collect(winners, win, NWIN, " ");
                collect(mynum, myn, NMY, " ");

                // printf("%i - ", winners[NWIN-1]);
                // printf("%i\n", mynum[NMY-1]);
                
                // Count the winning numbers
                
                for (int i = 0; i < NMY; i++) {
                        if (find(winners, mynum[i], NWIN)) {
                                cards[nc+win_offset] += cards[nc]; 
                                win_offset++;
                                // printf("*");
                                if (line_score == 0) line_score = 1;
                                else line_score *= 2;
                        }
                }
                // printf(" score: %i\n", line_score);
                win_offset = 1;
                nc++;
                sum += line_score;
                line_score = 0;
        }
        int sum2 = 0;
        for (int i = 0; i < LINES; i++) {
                sum2 += cards[i];
        }

        printf("Result:\n");
        printf("%i\n", sum2);
        return 0;

        return 0;
}

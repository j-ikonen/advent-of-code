#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>

#define MAX 160

struct map {
        long dest;
        long start;
        long end;
        long offset;
        long size;
};

int main() {
        long sum = 0;
        
        FILE *fptr;
        fptr = fopen("input", "r");
        if (!fptr) {
                printf("Input file could not be opened.");
                exit(0);
        }

        char line[MAX];
        long seeds[22];
        int seedi = 0;
        char seedline[220];
        struct map map[100];
        int mapi = 0;

        // Collect the seeds
        fgets(seedline, 220, fptr);
        char *token = strtok(seedline, ": ");
        while ((token = strtok(NULL, ": "))) {
                if (isdigit(token[0])) {
                        seeds[seedi++] = strtoul(token, NULL, 10);
                }
        }
        
        printf("seeds: ");
        for (int i = 0; i < seedi; i++) {
                printf("%lu, ", seeds[i]);
        }
        printf("i:%i\n", seedi);

        fgets(line, MAX, fptr); // Clear the empty line
        fgets(line, MAX, fptr); // Clear the title line

        for (int step = 0; step < 7; step++) {
                while (fgets(line, MAX, fptr)) {
                        if (isdigit(line[0])) {
                                struct map m;
                                m.dest = strtol(strtok(line, " "), NULL, 10);
                                m.start = strtol(strtok(NULL, " "), NULL, 10);
                                m.size = strtol(strtok(NULL, " "), NULL, 10);
                                m.offset = (long)m.dest - (long)m.start;
                                m.end = m.start + m.size;
                                map[mapi++] = m;
                                printf(" map: %li, %li, %li, offset: %li\n", m.dest, m.start, m.size, m.offset);
                        // Compute and clear the map 
                        } else { break; }
                }
                printf("mapi: %i\n", mapi);
                for (int si = 0; si < seedi; si++) {
                        for (int mi = 0; mi < mapi; mi++) {
                                // Seed is in range of this mapping
                                if (map[mi].start <= seeds[si] && seeds[si] < map[mi].end) {
                                        printf("si: %i, seed:%li, mi: %i, os: %li\n", si, seeds[si], mi, map[mi].offset);
                                        // seeds[si] = seeds[si] + map[mi].offset;
                                        seeds[si] = map[mi].dest + seeds[si] - map[mi].start;
                                        break;
                                }
                        }
                }
                printf("seeds: ");
                for (int i = 0; i < seedi; i++) {
                        printf("%lu, ", seeds[i]);
                }
                printf("\n");

                mapi = 0;
                fgets(line, MAX, fptr); // Clear the empty line
        }

        sum = seeds[0];
        for (int i = 0; i < seedi; i++) {
                // printf("%i\n", seeds[i]);
                sum = seeds[i] < sum ? seeds[i] : sum;
        }
        printf("Result:\n");
        printf("%lu\n", sum);
        return 0;

        return 0;
}

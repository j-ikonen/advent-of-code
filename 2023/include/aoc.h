#ifndef AOC_H__
#define AOC_H__

#include <string.h>
#include <stdlib.h>

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

#endif // AOC_H__
#include <stdio.h>
#include <math.h>

#include "q1.h"

int get_nth_digit(int start, int n) {
    long long current = start;
    while (n > 0) {
        ++current; 
        n -= log10(current);
    }

    char num[(int) log10(current) + 1];
    sprintf(num, "%lld", current);
    printf("%s\n", num);
    return num[-n] - '0';
}

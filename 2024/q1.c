#include <stdio.h>
#include <math.h>

#include "q1.h"

int get_nth_digit(int start, int n) {
    int current = start;
    while (n > 0) {
        ++current; 
        n -= log10(current);
    }

    char num[(int) log10(current) + 1];
    sprintf(num, "%d", current);
    return num[n] - '0';
}

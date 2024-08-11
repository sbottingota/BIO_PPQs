#include <stdio.h>
#include "q1.h"

#define LIMIT 101
#define TARGET 5

int main() {
    int sum = 0;

    for (int i = 0; i < LIMIT; ++i) {
        if (get_nth_digit(1, i) == TARGET) ++sum;
    }

    printf("%d\n", sum);
}


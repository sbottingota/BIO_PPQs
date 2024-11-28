#include <stdio.h>

#include "q1.h"

int main() {
    printf("Enter input:\n");
    int start, index;
    scanf("%d %d", &start, &index);

    printf("%d\n", get_nth_digit(start, index - 1));
}


#include <stdio.h>
#include <string.h>
#include <math.h>

int get_nth_digit(int start, int n) {
    int current = start;
    while (n > 0) {
        ++current; 
        n -= log10(current);
    }

    printf("%d\n", n);

    char num[(int) log10(current) + 1];
    sprintf(num, "%d", current);
    return num[n] - '0';
}

int main() {
    printf("Enter input:\n");
    int start, index;
    scanf("%d %d", &start, &index);

    printf("%d\n", get_nth_digit(start, index - 1));
}


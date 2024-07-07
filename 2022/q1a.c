#include <stdio.h>

#include "q1.h"

// decrypt the given string
int main() {
    char str[MAX_STRING_SIZE] = {0};

    printf("Enter the encrypted str: ");
    scanf("%s", str);

    decrypt(str);

    printf("\n%s\n", str);
}


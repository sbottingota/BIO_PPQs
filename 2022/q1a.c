#include <stdio.h>
#include <stdlib.h>

#include "q1.h"

int main() {
    char *str = malloc(MAX_STRING_SIZE);

    printf("Enter the encrypted str: ");
    scanf("%s", str);

    decrypt(str);

    printf("\n%s\n", str);

    free(str);
}


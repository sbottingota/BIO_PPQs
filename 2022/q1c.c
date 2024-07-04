#include <stdio.h>
#include <string.h>

#include "q1.h"

int main() {
    char original_str[] = "OLYMPIAD";
    char encrypted_str[] = "OLYMPIAD";
    
    int count = 0;

    do {
        decrypt(encrypted_str);
        ++count;
    } while (strcmp(original_str, encrypted_str) != 0);

    printf("%d\n", count);
}


#include <string.h>

#include "q1.h"

int indexof(char *str, char c) {
    return strchr(str, c) - str;
}

void decrypt(char *str) {
    int len = strlen(str);
    for (int i = len - 1; i > 0; --i) {
        int index = indexof(ALPHABET, str[i]) - indexof(ALPHABET, str[i - 1]) - 1;

        if (index < 0) {
            index += ALPHABET_LENGTH;
        }

        str[i] = ALPHABET[index % ALPHABET_LENGTH];
    }
}


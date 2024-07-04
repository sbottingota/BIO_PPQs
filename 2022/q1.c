#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STRING_SIZE 11  // 10 characters, plus null.

#define ALPHABET "ABCDEFGHIJKLMNOPQRSTUVWXYZ" // whitespace for offset (as in the question indices begin at 1)
#define ALPHABET_LENGTH 26

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

int main() {
    char *str = malloc(MAX_STRING_SIZE);

    printf("Enter the encrypted str: ");
    scanf("%s", str);

    decrypt(str);

    printf("\n%s\n", str);

    free(str);
}

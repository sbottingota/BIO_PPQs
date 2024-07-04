#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "q1.h"

#define PERMUTATION_LENGTH 5


// print all 5 letter strings where the decrypted version is the same as the encrypted version
int main() {
    char *original_str = malloc(PERMUTATION_LENGTH * sizeof(char));
    char *new_str = malloc(PERMUTATION_LENGTH * sizeof(char));

    for (int c0 = 0; c0 < ALPHABET_LENGTH; ++c0) {
        for (int c1 = 0; c1 < ALPHABET_LENGTH; ++c1) {
            for (int c2 = 0; c2 < ALPHABET_LENGTH; ++c2) {
                for (int c3 = 0; c3 < ALPHABET_LENGTH; ++c3) {
                    for (int c4 = 0; c4 < ALPHABET_LENGTH; ++c4) {
                        original_str[0] = ALPHABET[c0];
                        original_str[1] = ALPHABET[c1];
                        original_str[2] = ALPHABET[c2];
                        original_str[3] = ALPHABET[c3];
                        original_str[4] = ALPHABET[c4];

                        strcpy(new_str, original_str);

                        decrypt(new_str);

                        if (strcmp(original_str, new_str) == 0) {
                            printf("%s\n", original_str);
                        }
                    }
                }
            }
        }
    }

    free(original_str);
    free(new_str);
}

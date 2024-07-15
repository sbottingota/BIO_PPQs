#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "q3.h"

#define INPUT_STR_SIZE 13

void parse_state(char *str, State *state) {
    state->steps_so_far = 0;

    char *token = strtok(str, " ");

    for (int i = 0; i < N_SPIRES; ++i) {
        if (token == NULL) return;
        for (int j = 0; j < N_BALLS; ++j) {
            if (token[j] == '\0' || token[j] == '0') {
                break;
            } 
            char str[2] = {token[j], '\0'};
            state->val[i][j] = atoi(str);
        }

        token = strtok(NULL, " ");
    }
}

int main() {
    char start_str[INPUT_STR_SIZE] = {0};
    char end_str[INPUT_STR_SIZE] = {0};

    printf("Enter states:\n");
    fgets(start_str, INPUT_STR_SIZE, stdin);
    fgets(end_str, INPUT_STR_SIZE, stdin);

    State start_state = {0};
    State end_state = {0};
    parse_state(start_str, &start_state);
    parse_state(end_str, &end_state);

    printf("%d\n", find_min_moves(start_state, end_state));
}


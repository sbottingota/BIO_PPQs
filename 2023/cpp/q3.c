#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

#include "q3.h"

#define N_NEXT_STATE_ITEMS (N_SPIRES * (N_SPIRES - 1))


typedef struct QueueNode QueueNode;

struct QueueNode {
    State *state;
    QueueNode *next;
};


void enqueue(QueueNode **head, State *state) {
    QueueNode *new_node = malloc(sizeof(QueueNode));
    new_node->state = state;
    new_node->next = *head;

    *head = new_node;
}

State *dequeue(QueueNode **head) {
    QueueNode *current, *prev = NULL;

    State *ret = NULL;

    if (*head == NULL) return NULL;

    current = *head;

    while (current->next != NULL) {
        prev = current;
        current = current->next;
    }

    ret = current->state;
    free(current);
    
    if (prev != NULL) {
        prev->next = NULL;
    } else {
        *head = NULL;
    }

    return ret;
}

void free_all(QueueNode **head) {
    State *item;
    do {
        item = dequeue(head);
    } while (item != NULL);
}

int *top(int *spire) {
    int *p = spire;
    do {
        ++p;

        if (p == spire + (N_BALLS)) return NULL;  // check for full spire
    } while (*p != 0);

    return p - 1;
}

void move(State *start_state, int start_spire, int end_spire) {
    if (start_spire == end_spire) return;

    int *start = top(start_state->val[start_spire]);
    int *end = top(start_state->val[end_spire]);

    if (start == NULL || end == NULL) return;
    if (*start == 0) return;
    if (*end != 0) {
        ++end;
    }

    *end = *start;
    *start = 0;
}

// returns a State[] with the length N_NEXT_STATE_ITEMS
State *get_possible_next_states(State start_state) {
    State *possible_next_states = calloc(N_NEXT_STATE_ITEMS, sizeof(State));

    State *ptr = possible_next_states;
    
    for (int start_spire = 0; start_spire < N_SPIRES; ++start_spire) {
        for (int end_spire = 0; end_spire < N_SPIRES; ++end_spire) {
            if (start_spire == end_spire) continue;

            memcpy(ptr, &start_state, sizeof start_state);
            move(ptr, start_spire, end_spire);
            ++(ptr->steps_so_far);
            ++ptr;
        }
    }

    return possible_next_states;
}

bool are_states_equal(State *state1, State *state2) {
    for (int i = 0; i < N_SPIRES; ++i) {
        for (int j = 0; j < N_SPIRES; ++j) {
            if (state1->val[i][j] != state2->val[i][j]) {
                return false;
            }
        }
    }
    
    return true;
}

// find the minimum moves it takes to get between two states.
int find_min_moves(State start_state, State end_state) {
    if (are_states_equal(&start_state, &end_state)) return 0;

    QueueNode *states_head = NULL;

    enqueue(&states_head, &start_state);

    State *next_states;

    while (true) {
        next_states = get_possible_next_states(*dequeue(&states_head));

        for (int i = 0; i < N_NEXT_STATE_ITEMS; ++i) {
            if (are_states_equal(&(next_states[i]), &end_state)) {
                int steps_taken = next_states[i].steps_so_far; // save value before free'd
                free_all(&states_head);
                free(next_states);
                return steps_taken;
            }
            
            State *state_copy = malloc(sizeof(State));
            memcpy(state_copy, &(next_states[i]), sizeof(State));
            enqueue(&states_head, state_copy);
        }

        free(next_states);
    }
}
    

#define N_BALLS 4
#define N_SPIRES 4

struct State {
    int val[N_SPIRES][N_BALLS];
    int steps_so_far;
};
typedef struct State State;

int find_min_moves(State start_state, State end_state);

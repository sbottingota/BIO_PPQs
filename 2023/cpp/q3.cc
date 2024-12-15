#include <queue>
#include <cstring>
#include <cstdlib>
#include <cctype>

#include "q3.h"

using namespace std;


State State::fromString(char *str) {
    vector<vector<int>> stateValues(N_SPIRES, vector<int>(N_BALLS, 0));

    char *token = strtok(str, " ");

    for (int i = 0; i < N_SPIRES; ++i) {
        if (token == NULL) {
            break;
        }
    
        for (int j = 0; j < N_BALLS; ++j) {
            if (token[j] == '\0' || token[j] == '0') {
                break;
            } 
            stateValues[i][j] = token[j] - '0';  // digit value of token[j]
        }

        token = strtok(NULL, " ");
    }

    return State(stateValues);
}


State::State(vector<vector<int>> val, int movesSoFar) {
    this->movesSoFar = movesSoFar;

    this->val = val;
}

State State::moved(int fromSpire, int toSpire) {
    // check if origin spire is empty or destination spire is full
    if ((*this)[fromSpire][0] == 0 || (*this)[toSpire][N_BALLS-1] != 0) return *this;

    vector<vector<int>> newVal = val;

    int *from = &(newVal[fromSpire][0]) + N_BALLS-1;    
    int *to = &(newVal[toSpire][0]);

    while (*from == 0) --from;
    while (*to != 0) ++to;

    *to = *from;
    *from = 0;

    return State(newVal, movesSoFar + 1);
}

void State::computePossibleNextMoves(void) {
    for (int fromSpire = 0; fromSpire < N_SPIRES; ++fromSpire) {
        for (int toSpire = 0; toSpire < N_SPIRES; ++toSpire) {
            possibleNextMoves.push_back(moved(fromSpire, toSpire));
        }
    }
}

vector<State> State::getPossibleNextMoves(void) {
    if (possibleNextMoves.empty()) {
        computePossibleNextMoves();
    }

    return possibleNextMoves;
}

int State::getMovesSoFar(void) {
    return movesSoFar;
}

vector<int> State::operator[](int i) {
    return val.at(i);
}

bool State::operator==(State other) {
    for (int i = 0; i < N_SPIRES; ++i) {
        for (int j = 0; j < N_SPIRES; ++j) {
            if ((*this)[i][j] != other[i][j]) return false;
        }
    }

    return true;
}


int findMinMoves(State startState, State endState) {
    if (startState == endState) return 0;

    queue<State> nextStates;
    nextStates.push(startState);

    while (true) {
        State nextState = nextStates.front();
        nextStates.pop();
        
        for (State& state : nextState.getPossibleNextMoves()) {
            if (state == endState) {
                return state.getMovesSoFar();
            }

            nextStates.push(state);
        }
    }

}

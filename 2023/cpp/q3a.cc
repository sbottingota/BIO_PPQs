#include <iostream>

#include "q3.h"

#define MAX_INPUT_LEN 12 // size of "1234 0 0 0\n\0" or equivelent

using namespace std;


int main() {
    char startStr[MAX_INPUT_LEN], endStr[MAX_INPUT_LEN];

    cout << "Enter the states:" << endl;
    cin.getline(startStr, MAX_INPUT_LEN);
    cin.getline(endStr, MAX_INPUT_LEN);

    State startState = State::fromString(startStr);
    State endState = State::fromString(endStr);

    cout << findMinMoves(startState, endState) << endl;
}


#include <iostream>
#include <algorithm>

#include "q3.h"

#define MIN_MOVES 6

int main() {
    std::string boxes = "ABCDE";

    do {
        if (find_min_moves(boxes) == MIN_MOVES) {
            std::cout << boxes << "\n";
        }
    } while (std::next_permutation(boxes.begin(), boxes.end()));
    
    return 0;
}


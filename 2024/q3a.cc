#include <iostream>
#include <string>

#include "q3.h"

int main() {
    std::string word;
    std::cout << "Enter input:\n";
    std::cin >> word;

    std::cout << get_word_position(word) + 1 << '\n'; // +1 as question wants 1 based indices.
}


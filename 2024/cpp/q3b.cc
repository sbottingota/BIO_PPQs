#include <iostream>
#include <string>

#include "q3.h"

int main() {
    std::string word = "ACE";
    std::cout << generate_words_of_score(get_word_score(word))[0] << '\n';
}

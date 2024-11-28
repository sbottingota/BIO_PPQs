#include <string>
#include <vector>

#include "q3.h"

inline int get_letter_score(char letter) {
    return letter - 'A' + 1;
}

bool is_valid_word(std::string word) {
    for (int i = 0; i < word.length() - 1; ++i) {
        if (word[i] == word[i + 1]) return false;
    }
    return true;
}

std::vector<std::string> generate_words_of_score(int score) {
    std::vector<std::string> words;

    for (char letter = 'A'; letter <= 'Z'; ++letter) {
        if (get_letter_score(letter) == score) {
            words.push_back(std::string(1, letter));
            break;
        }

        std::vector<std::string> subwords = generate_words_of_score(score - get_letter_score(letter));
        for (std::string subword : subwords) {
            subword.insert(0, 1, letter);
            if (is_valid_word(subword)) {
                words.push_back(subword);
            }
        }
    }

    return words;
}

int get_word_score(std::string word) {
    int score = 0;
    for (char& c : word) {
        score += get_letter_score(c);
    }
    return score;
}

int get_word_position(std::string word) {
    std::vector<std::string> words = generate_words_of_score(get_word_score(word));

    for (int i = 0; i < words.size(); ++i) {
        if (words[i] == word) return i;
    }

    return -1;
}

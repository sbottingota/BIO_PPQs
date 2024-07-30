#include <algorithm>

#include "q1.h"

const std::string ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";


bool is_pat(std::string str) {
    if (str.length() == 1) return true;

    std::string left, right;
    for (int seperator = 1; seperator < str.length(); ++seperator) {
        left = str.substr(0, seperator);
        right = str.substr(seperator, str.length() - seperator);

        char min_left = ALPHABET[ALPHABET.length() - 1];
        for (char c : left) {
            if (c < min_left) {
                min_left = c;
            }
        }

        char max_right = ALPHABET[0];
        for (char c : right) {
            if (c > max_right) {
                max_right = c;
            }
        }

        std::reverse(left.begin(), left.end());
        std::reverse(right.begin(), right.end());

        if (max_right < min_left) {
            return is_pat(left) && is_pat(right);
        } 
    }

    return false;
}


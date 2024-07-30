#include <iostream>
#include <algorithm>

#include "q1.h"

int main() {
    std::string str = "ABCD";

    do {
        if (is_pat(str)) {
            std::cout << str << "\n";
        }
    } while (std::next_permutation(str.begin(), str.end()));
    
    return 0;
}

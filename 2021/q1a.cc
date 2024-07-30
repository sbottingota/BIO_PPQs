#include <iostream>

#include "q1.h"

inline std::string yes_or_no(bool val) {
    return val ? "YES" : "NO";
}

int main() {
    std::string s1, s2;
    std::cin >> s1 >> s2;

    std::cout << yes_or_no(is_pat(s1)) << std::endl;
    std::cout << yes_or_no(is_pat(s2)) << std::endl;
    std::cout << yes_or_no(is_pat(s1.append(s2))) << std::endl;

    return 0;
}
    

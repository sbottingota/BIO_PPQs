#include <iostream>

#include "q3.h"

int main() {
    std::string display_order;
    std::cin >> display_order;
    
    std::cout << find_min_moves(display_order) << "\n";

    return 0;
}


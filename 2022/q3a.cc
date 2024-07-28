#include <iostream>

#include "q3.h"


int main() {
    std::string cars;
    int n;

    std::cin >> cars >> n;

    std::cout << get_nth_preference_list(cars, n) << std::endl;

    return 0;
}

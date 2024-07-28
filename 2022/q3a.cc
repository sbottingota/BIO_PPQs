#include <iostream>

#include "q3.h"


int main() {
    std::string cars;
    int preference_list_index;

    std::cin >> cars >> preference_list_index;

    std::vector<std::string> preference_lists = get_preference_lists(cars);

    std::cout << preference_lists[preference_list_index - 1] << std::endl;

    return 0;
}

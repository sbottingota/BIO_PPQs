#include <cmath>

#include "q3.h"

#define UPPER std::string("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
#define LOWER std::string("abcdefghijklmnopqrstuvwxyz")


bool is_preference_list_valid(std::string preference_list, std::string cars) {
    std::string parked_cars(preference_list.length(), ' ');
    
    for (size_t car_index = 0; car_index < cars.length(); ++car_index) {
        char car = LOWER[car_index];
    
        int park_index = UPPER.find(preference_list[car_index]);

        while (isalpha(parked_cars[park_index])) {
            ++park_index;
        }

        if (cars[park_index] != car) return false;

        parked_cars[park_index] = car;
    }

    return true;
}

std::vector<std::string> get_preference_lists(std::string cars) {
    std::vector<std::string> ret, combinations;
    combinations = get_combinations(UPPER.substr(0, cars.length()));
    for (auto combination : combinations) {
        if (is_preference_list_valid(combination, cars)) {
            ret.push_back(combination);
        }
    }

    return ret;        
}

// get all combinations of string; for example: ABC -> AAA, AAB, AAC, ABA, ABB, etc.
std::vector<std::string> get_combinations(std::string charset) {
    int charset_len = charset.length();

    std::vector<std::string> ret;
    for (int i = 0; i < std::pow(charset_len, charset_len); ++i) {
        ret.push_back(std::string(charset_len, ' '));

        int charset_index = i;

        for (int j = charset_len - 1; j >= 0; --j) {
            ret[i][j] = charset[charset_index % charset_len];
            charset_index -= charset_index % charset_len;
            charset_index /= charset_len;
        }
    }

    return ret;
}


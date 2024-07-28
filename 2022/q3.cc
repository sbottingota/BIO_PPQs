#include <cmath>

#include "q3.h"

#define UPPER std::string("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
#define LOWER std::string("abcdefghijklmnopqrstuvwxyz")


bool is_preference_list_valid(std::string preference_list, std::string cars) {
    std::string parked_cars(preference_list.length(), ' ');
    
    for (int car_index = 0; car_index < cars.length(); ++car_index) {
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

std::string get_nth_preference_list(std::string cars, int n) {
    std::string combination;
    std::string charset = UPPER.substr(0, cars.length());

    int i = 0;
    
    while (n > 0) {
        do {
            combination = get_nth_combination(charset, i);
            ++i;
        } while (!is_preference_list_valid(combination, cars));
        --n;
    }

    return combination;        
}

// get nth combinations of string; for example: ABC, 2 -> AAC (AAA, AAB, **AAC**, ABA, etc.)
std::string get_nth_combination(std::string charset, int n) {
    std::string ret(charset.length(), ' ');

    for (int i = charset.length() - 1; i >= 0; --i) {
        ret[i] = charset[n % charset.length()];
        n -= n % charset.length();
        n /= charset.length();
    }

    return ret;
}


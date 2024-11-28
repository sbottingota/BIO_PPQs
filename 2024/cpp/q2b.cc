#include <iostream>
#include "q2.h"

#define LIMIT 10000
#define TARGET 2

int main() {
    auto fun = combined(T, T);

    int sum = 0;
    for (int i = 1; i <= LIMIT; ++i) {
        if (fun(i) == TARGET) ++sum;
    }

    std::cout << sum << '\n';
}
    

#include <functional>
#include "q2.h"

int E(int n) {
    return n * 2;
}

int O(int n) {
    return n * 2 - 1;
}

int T(int n) {
    int x = 0;
    
    while (n > 0) {
        for (int i = 0; i <= x; ++i) --n;
        ++x;
    }

    return x;
}

std::function<int(int)> combined(std::function<int(int)> f, std::function<int(int)> g) {
    return [f, g](int n) { return g(f(g(n))); };
}



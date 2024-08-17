#include <functional>
#include "q2.h"

long long E(long long n) {
    return n * 2;
}

long long O(long long n) {
    return n * 2 - 1;
}

long long T(long long n) {
    long long x = 0;
    
    while (n > 0) {
        for (long long i = 0; i <= x; ++i) --n;
        ++x;
    }

    return x;
}

std::function<long long(long long)> combined(std::function<long long(long long)> f, std::function<long long(long long)> g) {
    return [f, g](long long n) { return g(f(g(n))); };
}



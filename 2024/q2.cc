#include <functional>
#include <cstdint>

#include "q2.h"

std::uint64_t E(std::uint64_t n) {
    return n * 2;
}

std::uint64_t O(std::uint64_t n) {
    return n * 2 - 1;
}

std::uint64_t T(std::uint64_t n) {
    std::uint64_t x = 0;
    
    while (n > 0) {
        for (std::uint64_t i = 0; i <= x; ++i) --n;
        ++x;
    }

    return x;
}

std::function<std::uint64_t(std::uint64_t)> combined(std::function<std::uint64_t(std::uint64_t)> f, std::function<std::uint64_t(std::uint64_t)> g) {
    return [f, g](std::uint64_t n) { return g(f(g(n))); };
}



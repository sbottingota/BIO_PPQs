#include <functional>
#include <cstdint>
#include <cmath>

#include "q2.h"

std::uint64_t E(std::uint64_t n) {
    return n * 2;
}

std::uint64_t O(std::uint64_t n) {
    return n * 2 - 1;
}

std::uint64_t T(std::uint64_t n) {
    return std::round(std::sqrt(2 * n));
}

std::function<std::uint64_t(std::uint64_t)> combined(std::function<std::uint64_t(std::uint64_t)> f, std::function<std::uint64_t(std::uint64_t)> g) {
    return [f, g](std::uint64_t n) { return g(f(g(n))); };
}



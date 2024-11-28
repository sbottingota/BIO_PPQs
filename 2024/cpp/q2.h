#ifndef Q2_H
#define Q2_H

#include <functional>
#include <cstdint>

std::uint64_t E(std::uint64_t n);
std::uint64_t O(std::uint64_t n);
std::uint64_t T(std::uint64_t n);
std::function<std::uint64_t(std::uint64_t)> combined(std::function<std::uint64_t(std::uint64_t)> f, std::function<std::uint64_t(std::uint64_t)> g);

#endif

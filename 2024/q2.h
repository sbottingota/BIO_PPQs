#include <functional>

long long E(long long n);
long long O(long long n);
long long T(long long n);
std::function<long long(long long)> combined(std::function<long long(long long)> f, std::function<long long(long long)> g);


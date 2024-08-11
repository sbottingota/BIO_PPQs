#include <functional>

int E(int n);
int O(int n);
int T(int n);
std::function<int(int)> combined(std::function<int(int)> f, std::function<int(int)> g);


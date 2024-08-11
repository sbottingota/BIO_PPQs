#include <iostream>
#include <functional>
#include <string>

#include "q2.h"

std::function<int(int)> parse_char(char c) {
    switch (c) {
        case 'E': return E;
        case 'O': return O;
        case 'T': return T;
    }
}

std::function<int(int)> parse_string(std::string str) {
    auto fun = parse_char(str[0]);
    for (int i = 1; i < str.length(); ++i) {
        if (str[i] == '(') {
            std::string substring = str.substr(i);
            int end = substring.find(")");
            substring = substring.substr(1, end - 1);
            
            fun = combined(fun, parse_string(substring));

            i += end;
        } else {
            fun = combined(fun, parse_char(str[i]));
        }
    }
    return fun;
}

int main() {
    std::string str;
    int i;

    std::cout << "Enter input:\n";
    std::cin >> str >> i;

    auto fun = parse_string(str);

    std::cout << fun(i) << '\n';
}
    

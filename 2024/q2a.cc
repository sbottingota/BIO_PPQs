#include <iostream>
#include <functional>
#include <string>

#include "q2.h"

std::function<long long(long long)> parse_char(char c) {
    switch (c) {
        case 'E': return E;
        case 'O': return O;
        case 'T': return T;
        default:
            std::cerr << "Illegal value for argument c: '" << c << "'\n";
            exit(1);
    }
}

std::string get_first_bracketed_substr(std::string str) {
    int end = str.find(")");
    return str.substr(1, end - 1);
}

std::function<long long(long long)> parse_string(std::string str) {
    std::function<long long(long long)> fun;
    int i;

    if (str[0] == '(') {
        std::string substring = get_first_bracketed_substr(str);
        fun = parse_string(substring);
        i = substring.length();
    } else {
        fun = parse_char(str[0]);
        i = 1;
    }

    for (; i < str.length(); ++i) {
        if (str[i] == ')') continue;
        else if (str[i] == '(') {
            std::string substring = str.substr(i);
            substring = get_first_bracketed_substr(substring);
            fun = combined(fun, parse_string(substring));

            i += substring.length();
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
    

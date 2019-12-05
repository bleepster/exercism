#if !defined(REVERSE_STRING_H)
#define REVERSE_STRING_H

#include <string>
#include <algorithm>

namespace reverse_string {

std::string reverse_string(const std::string &str) {
    std::string reversed(str);
    std::reverse_copy(std::begin(str), std::end(str), std::begin(reversed));
    return reversed;
}

}

#endif

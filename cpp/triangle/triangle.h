#if !defined(TRIANGLE_H)
#define TRIANGLE_H

#include <stdexcept>

namespace triangle {
enum class flavor { equilateral, isosceles, scalene };

template <typename T>
bool isValid(T a, T b, T c) {
  return ((a > 0 && b > 0 && c > 0) &&
          (a + b >= c && b + c >= a && c + a >= b));
}

template <typename T>
flavor kind(T a, T b, T c) {
  if (!isValid(a, b, c)) throw std::domain_error("not a triangle");
  if (a == b && b == c)
    return flavor::equilateral;
  else if (a == b || b == c || a == c)
    return flavor::isosceles;
  else if (a != b && b != c && a != c)
    return flavor::scalene;
  else
    // throw an error if all conditions above are not met
    // such condition may not exist, just being explicit
    throw std::domain_error("unknown kind");
}
}  // namespace triangle

#endif  // TRIANGLE_H

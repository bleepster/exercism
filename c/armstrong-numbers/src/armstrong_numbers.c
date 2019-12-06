#include <math.h>

#include "armstrong_numbers.h"

int is_armstrong_number(int candidate)
{
    int digits, q, r, res;

    digits = 1;
    for (q = candidate; (q /= 10) > 0; ++digits)
        ; /* lonely loop - it has "nobody" ;) */

    res = 0;
    for (q = candidate; q > 0; q /= 10) {
        r = q % 10;
        res += pow(r, digits);
    }

    return (res == candidate);
}

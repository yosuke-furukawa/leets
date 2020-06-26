const HALF_INT_MIN = -1073741824;
const MIN = -(2**31);
const MAX = (2 ** 31) -1

var divide = function(dividend, divisor) {
    if (dividend == MIN && divisor == -1) {
        return MAX;
    }

    /* We need to convert both numbers to negatives.
     * Also, we count the number of negatives signs. */
    var negatives = 2;
    if (dividend > 0) {
        negatives--;
        dividend = -dividend;
    }
    if (divisor > 0) {
        negatives--;
        divisor = -divisor;
    }
    var highestDouble = divisor;
    var highestPowerOfTwo = -1;
    while (highestDouble >= HALF_INT_MIN && dividend <= highestDouble + highestDouble) {
        highestPowerOfTwo += highestPowerOfTwo;
        highestDouble += highestDouble;
    }

    var quotient = 0;
    while (dividend <= divisor) {
        if (dividend <= highestDouble) {
            quotient += highestPowerOfTwo;
            dividend -= highestDouble;
        }
        highestPowerOfTwo >>= 1;
        highestDouble >>= 1;
    }

    if (negatives != 1) {
        return -quotient;
    }
    return quotient;
};

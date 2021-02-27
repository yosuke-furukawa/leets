const HALF_INT_MIN: i32 = std::i32::MIN / 2;
const MAX: i32 = std::i32::MAX;
const MIN: i32 = std::i32::MIN;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut dividend = dividend;
        let mut divisor = divisor;
        if dividend == MIN && divisor == -1 {
            return MAX;
        }

        let mut negatives = 2;
        if dividend > 0 {
            negatives -= 1;
            dividend = -dividend;
        }
        if divisor > 0 {
            negatives -= 1;
            divisor = -divisor;
        }

        let mut highest_double = divisor;
        let mut highest_power_of_two = -1;
        while highest_double >= HALF_INT_MIN && dividend <= highest_double + highest_double {
            highest_power_of_two += highest_power_of_two;
            highest_double += highest_double;
        }

        let mut quotient = 0;
        while dividend <= divisor {
            if dividend <= highest_double {
                quotient += highest_power_of_two;
                dividend -= highest_double;
            }
            highest_power_of_two >>= 1;
            highest_double >>= 1;
        }

        if negatives != 1 {
            return -quotient;
        }
        quotient
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::divide(10, 3));
}

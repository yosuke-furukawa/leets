use std::collections::HashSet;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let mut res = 0;
        let primes = HashSet::from([2, 3, 5, 7, 11, 13, 17, 19]);
        for n in left as u64..=right as u64 {
            let count_ones = n.count_ones();
            if primes.contains(&count_ones) {
                res += 1;
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::count_prime_set_bits(6, 10));
    println!("{}", Solution::count_prime_set_bits(10, 15));
}

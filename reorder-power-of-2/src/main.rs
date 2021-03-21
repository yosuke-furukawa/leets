use std::collections::HashMap;

impl Solution {
    fn digit(n: i32) -> i32 {
        let mut m = n;
        let mut digit = 0;
        while m > 0 {
            digit += 1;
            m /= 10;
        }
        digit
    }
    fn map(n: i32) -> HashMap<i32, i32> {
        let mut num = n;
        let mut result = HashMap::new();
        while num > 0 {
            let q = num % 10;
            *result.entry(q).or_insert(0) += 1;
            num /= 10;
        }
        result
    }
    pub fn reordered_power_of2(n: i32) -> bool {
        if n == 1 {
            return true;
        }
        let digit = Self::digit(n);
        let mut m = 2;
        let map = Self::map(n);
        while digit >= Self::digit(m) {
            let target = Self::map(m);
            if map == target {
                return true;
            }
            if m * 2 > 1_000_000_000 {
                return false;
            }
            m *= 2;
        }
        false
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::reordered_power_of2(1));
    println!("{}", Solution::reordered_power_of2(10));
    println!("{}", Solution::reordered_power_of2(46));
    println!("{}", Solution::reordered_power_of2(536870912));
}

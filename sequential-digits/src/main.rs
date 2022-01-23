use std::collections::HashSet;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let root = 123456789;
        let digits = 9;
        fn put_next(n: i32, digits: u32, results: &mut HashSet<i32>) {
            if digits == 2 {
                return;
            }
            let n1 = n / 10;
            let n2 = n % 10_i32.pow(digits as u32 - 1);
            results.insert(n1);
            results.insert(n2);
            put_next(n1, digits - 1, results);
            put_next(n2, digits - 1, results);
        }
        let mut set = HashSet::new();
        set.insert(root);
        put_next(root, digits, &mut set);
        let mut results: Vec<i32> = set.into_iter().collect();
        results.sort_unstable();
        results
            .into_iter()
            .filter(|&x| x >= low && x <= high)
            .collect()
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::sequential_digits(100, 300));
    println!("{:?}", Solution::sequential_digits(1000, 13000));
    println!("{:?}", Solution::sequential_digits(10, 1_000_000_000));
}

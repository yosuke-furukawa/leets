use std::cmp::min;
use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            *count.entry(n).or_insert(0) += 1;
        }

        let mut result = 0;
        for n in count.keys() {
            result += match n {
                n if *n > k / 2 => continue,
                n if (k as f64 / 2.0) != *n as f64 => {
                    *min(count.get(&n).unwrap(), count.get(&(k - n)).unwrap_or(&0))
                }
                _ => count.get(&n).unwrap() / 2,
            };
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_operations(vec![1, 2, 3, 4], 5));
    println!("{}", Solution::max_operations(vec![3, 1, 3, 4, 3], 6));
}

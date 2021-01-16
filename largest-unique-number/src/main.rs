use std::collections::HashSet;

impl Solution {
    pub fn largest_unique_number(a: Vec<i32>) -> i32 {
        let mut num = a;
        let mut exists = HashSet::new();
        num.sort_unstable_by(|a, b| b.cmp(a));
        for (i, v) in num.iter().enumerate() {
            let n = num.get(i + 1).unwrap_or(&-1);
            if v == n {
                exists.insert(v);
            } else if !exists.contains(v) {
                return *v;
            }
        }
        -1
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::largest_unique_number(vec![1, 2, 2, 3, 3, 3])
    );
}

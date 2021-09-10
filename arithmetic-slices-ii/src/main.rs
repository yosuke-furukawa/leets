use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut d: Vec<HashMap<i64, usize>> = Vec::with_capacity(n);
        for &b in &nums {
            let mut cur = HashMap::new();
            for (h, &a) in d.iter().zip(&nums) {
                let step = a as i64 - b as i64;
                *cur.entry(step).or_default() += h.get(&step).map_or(1, |&x| x + 1);
            }
            d.push(cur);
        }
        (d.iter().flat_map(|h| h.values()).sum::<usize>() - n * (n - 1) / 2) as i32
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10])
    );
}

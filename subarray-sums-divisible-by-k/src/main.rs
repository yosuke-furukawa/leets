use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix_sum = HashMap::new();
        prefix_sum.insert(0, 1);
        let mut count = 0;
        let mut premod = 0;
        for &n in nums.iter() {
            premod = (premod + n % k + k) % k;
            if let Some(b) = prefix_sum.get(&premod) {
                count += b;
            }
            *prefix_sum.entry(premod).or_insert(0) += 1;
        }
        count
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5)
    );
}

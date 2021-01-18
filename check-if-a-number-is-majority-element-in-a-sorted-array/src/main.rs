use std::collections::HashMap;

impl Solution {
    pub fn is_majority_element(nums: Vec<i32>, target: i32) -> bool {
        let mut count = HashMap::new();
        let mut max = 0;
        let mut res = -1;
        let len = nums.len() as i32;
        for n in nums {
            let v = count.entry(n).or_insert(0);
            *v += 1;
            if *v > max {
                max = *v;
                res = n;
            }
        }
        res == target && count.get(&target).unwrap() > &(len / 2)
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::is_majority_element(vec![2, 4, 5, 5, 5, 5, 5, 6, 6], 5)
    );
}

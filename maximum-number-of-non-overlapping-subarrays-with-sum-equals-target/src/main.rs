use std::collections::HashSet;

impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        let mut res = 0;
        let mut sum = 0;
        let mut set = HashSet::new();
        set.insert(0);
        for n in nums {
            sum += n;
            if set.contains(&(sum - target)) {
                res += 1;
                sum = 0;
                set.clear();
                set.insert(0);
                continue;
            }
            set.insert(sum);
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_non_overlapping(vec![1, 1, 1, 1, 1], 2));
    println!(
        "{}",
        Solution::max_non_overlapping(vec![-1, 3, 5, 1, 4, 2, -9], 6)
    );
}

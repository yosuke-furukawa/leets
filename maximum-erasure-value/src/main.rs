use std::collections::HashSet;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let n = nums.len();
        let mut set = HashSet::new();
        set.insert(nums[l]);
        let mut max = nums[l];
        let mut sum = nums[l];
        while l < n && r <= n {
            max = max.max(sum);
            if nums.get(r + 1).is_some() && !set.contains(&nums[r + 1]) {
                set.insert(nums[r + 1]);
                sum += nums[r + 1];
                r += 1;
            } else {
                set.remove(&nums[l]);
                sum -= nums[l];
                l += 1;
            }
        }
        max
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]));
    println!(
        "{}",
        Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5])
    );
}

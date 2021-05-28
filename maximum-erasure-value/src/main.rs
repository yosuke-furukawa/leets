use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let n = nums.len();
        let mut queue = VecDeque::new();
        queue.push_back(nums[l]);
        let mut set = HashSet::new();
        set.insert(nums[l]);
        let mut sum = 0;
        while l < n && r <= n {
            sum = sum.max(queue.iter().sum());
            if nums.get(r + 1).is_some() && !set.contains(&nums[r + 1]) {
                queue.push_back(nums[r + 1]);
                set.insert(nums[r + 1]);
                r += 1;
                continue;
            } else {
                let popped = queue.pop_front().unwrap();
                set.remove(&popped);
                l += 1;
            }
        }
        sum
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

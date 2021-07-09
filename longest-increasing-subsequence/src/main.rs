impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        dp[0] = 1;
        let mut max = 1;
        for i in 1..dp.len() {
            let mut val = 0;
            for j in 0..i {
                if nums[i] > nums[j] {
                    val = val.max(dp[j]);
                }
            }
            dp[i] = val + 1;
            max = max.max(dp[i]);
        }
        max
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18])
    );
}

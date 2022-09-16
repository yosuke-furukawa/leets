impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let mut dp = vec![0; multipliers.len() + 1];
        for (i, mi) in multipliers.into_iter().enumerate().rev() {
            for (j, (l, r)) in nums.iter().zip(&nums[nums.len() - 1 - i..]).enumerate() {
                dp[j] = std::cmp::max(mi * l + dp[j + 1], mi * r + dp[j]);
            }
            dp.truncate(i + 1);
        }
        dp[0]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::maximum_score(vec![1, 2, 3], vec![3, 2, 1]));
}

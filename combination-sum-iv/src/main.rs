impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; (target + 1) as usize];
        dp[0] = 1;

        for sum in 1..target + 1 {
            for num in nums.iter() {
                if sum - num >= 0 {
                    dp[sum as usize] += dp[(sum - num) as usize];
                }
            }
        }
        dp[target as usize]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::combination_sum4(vec![1, 2, 3], 4));
}

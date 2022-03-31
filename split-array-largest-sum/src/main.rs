impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut sum = vec![0; nums.len()];
        sum[0] = nums[0];
        for i in 1..nums.len() {
            sum[i] = sum[i - 1] + nums[i];
        }
        dp[0] = sum[sum.len() - 1];
        for i in 1..nums.len() {
            dp[i] = sum[sum.len() - 1] - sum[i - 1];
        }

        for i in 1..m as usize {
            for s in 0..nums.len() - i {
                for e in (s + 1)..=nums.len() - i {
                    let t = if s == 0 {
                        sum[e - 1]
                    } else {
                        sum[e - 1] - sum[s - 1]
                    };
                    dp[s] = dp[s].min(t.max(dp[e]));
                }
            }
        }
        dp[0]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::split_array(vec![7, 2, 5, 10, 8], 2));
}

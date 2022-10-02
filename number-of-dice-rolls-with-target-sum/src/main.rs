impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut dp = vec![vec![0; target as usize + 1]; n as usize + 1];
        dp[0][0] = 1;
        for i in 1..=n as usize {
            for j in 1..=target as usize {
                for l in 1..=k as usize {
                    if j >= l {
                        dp[i][j] += dp[i - 1][j - l];
                        dp[i][j] %= 1000000007;
                    }
                }
            }
        }
        dp[n as usize][target as usize]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::num_rolls_to_target(1, 6, 3));
}

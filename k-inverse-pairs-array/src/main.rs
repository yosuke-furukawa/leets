impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut dp = vec![vec![0; k + 1]; n + 1];
        for i in 1..=n {
            for j in 0..=k {
                if j == 0 {
                    dp[i][j] = 1;
                } else {
                    for p in 0..=(i - 1).min(j) {
                        dp[i][j] = (dp[i][j] + dp[i - 1][j - p]) % 1_000_000_007;
                    }
                }
            }
        }
        dp[n][k]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::k_inverse_pairs(3, 1));
}

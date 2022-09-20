impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        for i in 1..=s1.len() {
            dp[i][0] = dp[i - 1][0] + s1[i - 1] as i32;
        }
        for j in 1..=s2.len() {
            dp[0][j] = dp[0][j - 1] + s2[j - 1] as i32;
        }
        for i in 1..=s1.len() {
            for j in 1..=s2.len() {
                if s1[i - 1] == s2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j - 1] + s1[i - 1] as i32 + s2[j - 1] as i32;
                }
                dp[i][j] = dp[i][j].min(dp[i - 1][j] + s1[i - 1] as i32);
                dp[i][j] = dp[i][j].min(dp[i][j - 1] + s2[j - 1] as i32);
            }
        }
        dp[s1.len()][s2.len()]
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::minimum_delete_sum("sea".to_string(), "eat".to_string())
    );
}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![vec![0; text2.len()]; text1.len()];
        for i in 0..text1.len() {
            for j in 0..text2.len() {
                if text1.get(i..i + 1) == text2.get(j..j + 1) {
                    if i == 0 || j == 0 {
                        dp[i][j] = 1;
                        continue;
                    }
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    continue;
                }
                dp[i][j] = match (i, j) {
                    (_, _) if i < 1 && j < 1 => 0,
                    (_, _) if i < 1 => dp[i][j - 1],
                    (_, _) if j < 1 => dp[i - 1][j],
                    (_, _) => dp[i - 1][j].max(dp[i][j - 1]),
                };
            }
        }
        dp[text1.len() - 1][text2.len() - 1]
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string())
    );
}

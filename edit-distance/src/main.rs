impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let words1: Vec<char> = word1.chars().collect();
        let words2: Vec<char> = word2.chars().collect();
        let (n, m) = (words1.len(), words2.len());
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for i in 0..=n {
            dp[i][0] = i as i32;
        }
        for j in 0..=m {
            dp[0][j] = j as i32;
        }
        for i in 1..=n {
            for j in 1..=m {
                dp[i][j] = dp[i - 1][j - 1] + (words1[i - 1] != words2[j - 1]) as i32;
                dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
                dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1);
            }
        }
        dp[n][m]
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::min_distance("horse".to_string(), "ros".to_string())
    );
}

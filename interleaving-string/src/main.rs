impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s3.len() != s1.len() + s2.len() {
            return false;
        }
        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                dp[i][j] = match (i, j) {
                    (0, 0) => true,
                    (0, _) => dp[i][j - 1] && s2.get(j - 1..j) == s3.get(i + j - 1..i + j),
                    (_, 0) => dp[i - 1][j] && s1.get(i - 1..i) == s3.get(i + j - 1..i + j),
                    (_, _) => {
                        (dp[i - 1][j] && s1.get(i - 1..i) == s3.get(i + j - 1..i + j))
                            || (dp[i][j - 1] && s2.get(j - 1..j) == s3.get(i + j - 1..i + j))
                    }
                }
            }
        }
        dp[s1.len()][s2.len()]
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string()
        )
    );
    println!(
        "{}",
        Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbbaccc".to_string()
        )
    );
}

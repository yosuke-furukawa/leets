impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let len1 = word1.len();
        let len2 = word2.len();

        let mut dp: Vec<Vec<i32>> = vec![vec![0; len2 + 1]; len1 + 1];
        for i in 1..=len1 {
            for j in 1..=len2 {
                if word1.get(i - 1..i) == word2.get(j - 1..j) {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
                }
            }
        }
        (len1 + len2) as i32 - 2 * dp[len1][len2] as i32
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::min_distance("sea".to_string(), "eat".to_string())
    );
    println!(
        "{}",
        Solution::min_distance("leetcode".to_string(), "etco".to_string())
    );
}

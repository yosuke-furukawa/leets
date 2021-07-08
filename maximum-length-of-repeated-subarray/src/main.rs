impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len()]; nums1.len()];
        let mut max = 0;
        for (i, n1) in nums1.iter().enumerate() {
            for (j, n2) in nums2.iter().enumerate() {
                if n1 == n2 && (i == 0 || j == 0) {
                    dp[i][j] = 1;
                } else if n1 == n2 {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                }
                max = max.max(dp[i][j]);
            }
        }
        max
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7])
    );
    println!(
        "{}",
        Solution::find_length(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0])
    );
    println!(
        "{}",
        Solution::find_length(vec![0, 0, 0, 0, 1], vec![1, 0, 0, 0, 0])
    );
}

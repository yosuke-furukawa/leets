impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n as usize {
            for j in 1..=i {
                dp[i] += dp[i - j] * dp[j - 1];
            }
        }
        dp[n as usize]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::num_trees(3));
    println!("{}", Solution::num_trees(1));
}

const MAX: i32 = 1000000000;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![vec![MAX; (amount + 1) as usize]; coins.len() + 1];
        dp[0][0] = 0;
        for i in 0..dp.len() - 1 {
            for j in 0..dp[i].len() {
                if j >= coins[i] as usize {
                    dp[i + 1][j] = dp[i][j].min(dp[i + 1][j - coins[i] as usize] + 1);
                } else {
                    dp[i + 1][j] = dp[i][j];
                }
            }
        }
        let ans = dp[coins.len()][amount as usize];
        if ans == MAX {
            return -1;
        }
        ans
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::coin_change(vec![1, 2, 5], 11));
}

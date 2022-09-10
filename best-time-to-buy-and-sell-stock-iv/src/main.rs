impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len() as i32;
        if k > n / 2 {
            let mut profit = 0;
            for i in 1..n as usize {
                profit += (prices[i] - prices[i - 1]).max(0);
            }
            return profit;
        }

        let mut dp = vec![vec![0; n as usize]; (k + 1) as usize];
        for i in 1..=k as usize {
            let mut tmp_profit = -prices[0];
            for j in 1..n as usize {
                dp[i][j] = dp[i][j - 1].max(tmp_profit + prices[j]);
                tmp_profit = tmp_profit.max(dp[i - 1][j - 1] - prices[j]);
            }
        }

        dp[k as usize][n as usize - 1]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_profit(2, vec![2, 4, 1]));
}

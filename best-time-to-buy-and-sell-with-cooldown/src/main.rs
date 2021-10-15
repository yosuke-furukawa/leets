impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let n = prices.len();
        let mut stock = vec![0; n + 1];
        let mut money = vec![0; n + 1];
        stock[1] = -prices[0];
        money[1] = 0;
        for i in 1..n {
            stock[i + 1] = stock[i].max(money[i - 1] - prices[i]);
            money[i + 1] = (stock[i] + prices[i]).max(money[i]);
        }
        money[n]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_profit(vec![1, 2, 3, 0, 2]));
}

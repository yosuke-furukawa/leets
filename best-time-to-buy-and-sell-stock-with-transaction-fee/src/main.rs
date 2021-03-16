impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let len = prices.len();
        let mut buy = 0;
        let mut sell = -prices[0];

        for p in prices.iter().take(len).skip(1) {
            buy = buy.max(sell + p - fee);
            sell = sell.max(buy - p);
        }
        buy
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2));
}

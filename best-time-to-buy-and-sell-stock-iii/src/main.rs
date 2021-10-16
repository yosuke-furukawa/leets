impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut min_lr = prices.first().unwrap();
        let mut max_rl = prices.last().unwrap();
        let mut dp_lr = vec![0; prices.len()];
        let mut dp_rl = vec![0; prices.len()];
        let mut max = 0;
        for (i, price) in prices.iter().enumerate().skip(1) {
            if price < min_lr {
                min_lr = price;
            }
            dp_lr[i] = dp_lr[i - 1].max(price - min_lr);
        }

        for (i, price) in prices.iter().enumerate().rev().skip(1) {
            if price > max_rl {
                max_rl = price;
            }
            dp_rl[i] = dp_rl[i + 1].max(max_rl - price);
        }

        for i in 0..prices.len() {
            max = max.max(dp_lr[i] + dp_rl[i]);
        }
        max
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]));
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = std::i32::MAX;
        let mut max_profit = 0;
        for price in prices {
            if price < min_price {
                min_price = price;
            }
            if price - min_price > max_profit {
                max_profit = price - min_price;
            }
        }
        max_profit
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
}

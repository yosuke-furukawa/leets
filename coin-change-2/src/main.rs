impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; (amount + 1) as usize];
        dp[0] = 1;
        for coin in coins {
            for i in (coin as usize)..dp.len() {
                dp[i] += dp[i - coin as usize];
            }
        }
        dp[amount as usize]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::change(5, vec![1, 2, 5]));
    println!("{}", Solution::change(3, vec![2]));
    println!("{}", Solution::change(10, vec![10]));
}

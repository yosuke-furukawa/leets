const MOD: u64 = 1_000_000_007;

impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let mut res = 1_u64;
        for i in 1..=n as u64 {
            res = (res * i) % MOD;
            res = (res * (2 * i - 1)) % MOD;
        }
        res as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::count_orders(2));
    println!("{}", Solution::count_orders(500));
}

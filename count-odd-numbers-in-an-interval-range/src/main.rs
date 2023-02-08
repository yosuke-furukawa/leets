impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (high - low + 1) / 2 + (low % 2 + high % 2) / 2
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::count_odds(3, 7));
}

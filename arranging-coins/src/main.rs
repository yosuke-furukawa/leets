impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        (((n as f64 * 2.0) + 0.25).sqrt() - 0.5) as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::arrange_coins(5));
    println!("{}", Solution::arrange_coins(8));
    println!("{}", Solution::arrange_coins(1804289383));
}

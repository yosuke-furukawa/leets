impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && 2_i64.pow(31) % n as i64 == 0
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::is_power_of_two(1));
    println!("{}", Solution::is_power_of_two(16));
    println!("{}", Solution::is_power_of_two(3));
    println!("{}", Solution::is_power_of_two(342809482));
}

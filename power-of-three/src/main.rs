impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && 3_i32.pow(19) % n == 0
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::is_power_of_three(27));
}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        if num % 9 == 0 {
            return 9;
        }
        num % 9
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::add_digits(38));
}

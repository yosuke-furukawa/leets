impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        match n {
            2 => 1,
            3 => 2,
            n if n % 3 == 0 => 3_i32.pow(n as u32 / 3),
            n if n % 3 == 1 => 4 * 3_i32.pow((n as u32 - 4) / 3),
            _ => 2 * 3_i32.pow(n as u32 / 3),
        }
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::integer_break(2));
}

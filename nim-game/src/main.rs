impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::can_win_nim(4));
}

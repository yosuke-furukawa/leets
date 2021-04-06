impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        match n {
            _ if n % 2 == 0 => n * n / 4,
            _ => (n * n - 1) / 4,
        }
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_operations(3));
}

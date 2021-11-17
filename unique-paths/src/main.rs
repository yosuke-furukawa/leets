impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let w = m - 1;
        let h = n - 1;
        let mut w_h = w + h;
        let mut result = 1.0;
        for num in 1..=h {
            result *= w_h as f64;
            result /= num as f64;
            w_h -= 1;
        }
        result.round() as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::unique_paths(3, 7));
    println!("{}", Solution::unique_paths(10, 10));
    println!("{}", Solution::unique_paths(2, 100));
}

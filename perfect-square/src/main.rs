impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut n = n;
        while n % 4 == 0 {
            n /= 4;
        }
        if n % 8 == 7 {
            return 4;
        }
        if is_square(n) {
            return 1;
        }
        let mut i = 1;
        while i * i <= n {
            if is_square(n - i * i) {
                return 2;
            }
            i += 1;
        }
        3
    }
}

fn is_square(n: i32) -> bool {
    let k = (n as f64).sqrt() as i32;
    k * k == n
}

struct Solution;

fn main() {
    println!("{}", Solution::num_squares(12));
    println!("{}", Solution::num_squares(13));
}

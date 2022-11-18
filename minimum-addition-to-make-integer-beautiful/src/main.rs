impl Solution {
    fn sum_digits(n: i64) -> i32 {
        let mut n = n;
        let mut sum = 0;
        while n > 0 {
            sum += (n % 10) as i32;
            n /= 10;
        }
        sum
    }
    pub fn make_integer_beautiful(n: i64, target: i32) -> i64 {
        let m = n;
        let mut n = n;
        let mut p: f64 = 10.0;
        while Self::sum_digits(n) > target {
            n = ((n as f64 / p).ceil() * p) as i64;
            p *= 10.0;
        }
        n - m
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::make_integer_beautiful(16, 6));
    println!("{}", Solution::make_integer_beautiful(467, 6));
    println!("{}", Solution::make_integer_beautiful(1, 1));
}

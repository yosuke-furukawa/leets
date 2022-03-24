impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut count = 0;
        let mut i = 1;
        while i <= n {
            let divider = i * 10;
            count += (n / divider) * i + (n % divider - i + 1).max(0).min(i);
            i *= 10;
        }
        count
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::count_digit_one(13));
}

impl Solution {
    pub fn number_of_steps (num: i32) -> i32 {
        let mut n = num;
        let mut count = 0;
        while n > 0 {
            let m = (n as f64).log2();
            let mi = m as i32;
            if m - mi as f64 == 0.0 {
                count += mi + 1;
                break;
            }
            n = match n % 2 {
                0 => n / 2,
                _ => n - 1,
            };
            count += 1;
        }
        count
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::number_of_steps(14));
    println!("{}", Solution::number_of_steps(8));
    println!("{}", Solution::number_of_steps(123));
    println!("{}", Solution::number_of_steps(328));
}

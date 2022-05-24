impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        let mut sum = 1;
        for i in 2..=(num as f64).sqrt().floor() as i32 {
            if num % i == 0 {
                sum += i;
                sum += num / i;
            }
        }
        sum == num
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::check_perfect_number(6));
    println!("{}", Solution::check_perfect_number(7));
    println!("{}", Solution::check_perfect_number(28));
    println!("{}", Solution::check_perfect_number(496));
    println!("{}", Solution::check_perfect_number(8128));
}

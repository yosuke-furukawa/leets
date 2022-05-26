impl Solution {
    fn factorial(k: i32) -> i32 {
        let mut res = 1;
        for i in 1..=k {
            res *= i;
        }
        res
    }
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let mut res = 0;
        for i in 1..=n {
            res += 9 * Self::factorial(9) / Self::factorial(10 - i);
        }
        res + 1
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::count_numbers_with_unique_digits(2));
    println!("{}", Solution::count_numbers_with_unique_digits(1));
    println!("{}", Solution::count_numbers_with_unique_digits(8));
}

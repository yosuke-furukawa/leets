impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        (num as f64).log2() as i32 + num.count_ones() as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::number_of_steps(14));
    println!("{}", Solution::number_of_steps(8));
    println!("{}", Solution::number_of_steps(123));
    println!("{}", Solution::number_of_steps(328));
}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().reduce(|a, b| a ^ b).unwrap_or(0)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::single_number(vec![2, 2, 1]));
}

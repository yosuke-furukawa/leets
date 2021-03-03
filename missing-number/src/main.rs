impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        let exp = (len + 1) * len / 2;
        let sum: i32 = nums.iter().sum();
        exp - sum
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::missing_number(vec![3, 0, 1]));
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(n) => n as i32,
            _ => -1,
        }
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::search(vec![-1, 0, 3, 5, 9, 12], 9));
    println!("{}", Solution::search(vec![-1, 0, 3, 5, 9, 12], 2));
}

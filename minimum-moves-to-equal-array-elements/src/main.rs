impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        nums.iter().sum::<i32>() - nums.len() as i32 * nums.iter().min().unwrap()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_moves(vec![1, 2, 3]));
}

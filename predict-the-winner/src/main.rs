impl Solution {
    fn winner(nums: &[i32], i: usize, j: usize, turn: i32) -> i32 {
        if i == j {
            return turn * nums[i];
        }
        let a = turn * nums[i] + Self::winner(nums, i + 1, j, -turn);
        let b = turn * nums[j] + Self::winner(nums, i, j - 1, -turn);
        turn * (turn * a).max(turn * b)
    }
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        Self::winner(&nums, 0, nums.len() - 1, 1) >= 0
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::predict_the_winner(vec![1, 5, 2]));
}

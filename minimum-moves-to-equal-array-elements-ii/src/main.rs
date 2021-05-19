impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut nums = nums;
        nums.sort_unstable();
        let div = if len % 2 == 1 {
            nums[len / 2]
        } else {
            (nums[(len - 1) / 2] + nums[((len - 1) / 2) + 1]) / 2
        };
        nums.into_iter().fold(0, |acc, n| acc + (n - div).abs())
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_moves2(vec![1, 2, 3]));
    println!("{}", Solution::min_moves2(vec![1, 10, 2, 9]));
    println!("{}", Solution::min_moves2(vec![1, 2, 43]));
}

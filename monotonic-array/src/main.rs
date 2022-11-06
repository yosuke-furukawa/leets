impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut increasing = true;
        let mut decreasing = true;
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                increasing = false;
            }
            if nums[i] > nums[i - 1] {
                decreasing = false;
            }
        }
        increasing || decreasing
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::is_monotonic(vec![1, 2, 2, 3]));
}

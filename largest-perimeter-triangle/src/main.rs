impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        for i in 0..nums.len() - 2 {
            if nums[i] < nums[i + 1] + nums[i + 2] {
                return nums[i] + nums[i + 1] + nums[i + 2];
            }
        }
        0
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::largest_perimeter(vec![2, 1, 2]));
}

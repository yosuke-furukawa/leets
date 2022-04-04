impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let max = nums[n - 1] * nums[n - 2] * nums[n - 3];
        let min = nums[0] * nums[1] * nums[n - 1];
        max.max(min)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::maximum_product(vec![1, 2, 3]));
    println!("{}", Solution::maximum_product(vec![-1, -2, -3, -4]));
}

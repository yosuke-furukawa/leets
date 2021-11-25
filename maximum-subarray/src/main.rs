impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_current = nums[0];
        let mut max_global = nums[0];
        for n in nums.iter().skip(1) {
            max_current = (max_current + n).max(*n);
            if max_current > max_global {
                max_global = max_current;
            }
        }
        max_global
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
}

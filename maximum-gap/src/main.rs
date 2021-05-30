impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut max = 0;
        for window in nums.windows(2) {
            max = max.max(window[1] - window[0]);
        }
        max
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::maximum_gap(vec![3, 6, 9, 1]));
    println!("{}", Solution::maximum_gap(vec![10]));
}

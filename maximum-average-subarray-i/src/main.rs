impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut sum: i32 = nums[0..k].iter().sum();

        let mut max = sum;
        for i in k..nums.len() {
            sum += nums[i] - nums[i - k];
            if sum > max {
                max = sum;
            }
        }

        max as f64 / k as f64
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4)
    );
}

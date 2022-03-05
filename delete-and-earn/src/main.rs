impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        let mut buckets: Vec<i32> = vec![0; 10001];
        for i in 0..nums.len() {
            buckets[nums[i] as usize] += nums[i];
        }
        let mut dp: Vec<i32> = vec![0; buckets.len()];
        dp[1] = buckets[0].max(buckets[1]);
        for i in 2..buckets.len() {
            dp[i] = (buckets[i] + dp[i - 2]).max(dp[i - 1]);
        }
        dp[buckets.len() - 1]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::delete_and_earn(vec![3, 4, 2]));
}

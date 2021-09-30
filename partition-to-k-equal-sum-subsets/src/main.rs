impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut nums = nums;
        nums.sort_unstable();
        let sum = nums.iter().sum::<i32>();
        let target = sum / k;
        if sum % k > 0 || nums[n - 1] > target {
            return false;
        }

        let mut dp = vec![false; 1 << n];
        dp[0] = true;
        let mut total = vec![0; 1 << n];

        for state in 0..(1 << n) {
            if !dp[state] {
                continue;
            }

            for i in 0..n {
                let future = state | (1 << i);
                if state != future && !dp[future] {
                    if nums[i] <= target - (total[state] % target) {
                        dp[future] = true;
                        total[future] = total[state] + nums[i];
                    } else {
                        break;
                    }
                }
            }
        }
        dp[(1 << n) - 1]
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4)
    );
}

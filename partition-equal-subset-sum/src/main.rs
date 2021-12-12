impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut sum = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }

        sum /= 2;

        let mut dp = vec![vec![false; sum as usize + 1]; len];
        for d in dp.iter_mut() {
            d[0] = true;
        }
        for s in 1..=sum {
            if nums[0] == s {
                dp[0][s as usize] = true;
            }
        }

        for i in 1..len {
            for s in 1..=sum {
                if dp[i - 1][s as usize] {
                    dp[i][s as usize] = dp[i - 1][s as usize];
                } else if s >= nums[i] {
                    dp[i][s as usize] = dp[i - 1][(s - nums[i]) as usize];
                }
            }
        }

        dp[len - 1][sum as usize]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::can_partition(vec![1, 5, 11, 5]));
    println!("{}", Solution::can_partition(vec![1, 2, 3, 5]));
}

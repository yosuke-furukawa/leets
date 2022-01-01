impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut new_nums: Vec<i32> = vec![1];
        for n in nums {
            new_nums.push(n);
        }
        new_nums.push(1);
        let mut memo = vec![vec![0; new_nums.len()]; new_nums.len()];
        Self::dp(&mut memo, 0, new_nums.len() - 1, &new_nums)
    }

    fn dp(memo: &mut [Vec<i32>], left: usize, right: usize, nums: &[i32]) -> i32 {
        if left + 1 == right {
            return 0;
        }

        if memo[left][right] > 0 {
            return memo[left][right];
        }

        let mut result = 0;
        for i in left + 1..right {
            result = result.max(
                nums[left] * nums[i] * nums[right]
                    + Self::dp(memo, left, i, nums)
                    + Self::dp(memo, i, right, nums),
            );
        }
        memo[left][right] = result;
        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_coins(vec![3, 1, 5, 8]));
}

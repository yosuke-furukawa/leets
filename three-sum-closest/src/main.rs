impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut min = 1_000_000_000;
        let mut nums = nums;
        let mut result = 0;
        nums.sort_unstable();
        for i in 0..nums.len() {
            let mut i1 = i + 1;
            let mut i2 = nums.len() - 1;
            while i1 < i2 {
                let sum = nums[i] + nums[i1] + nums[i2];
                let diff = (target - sum).abs();
                if min > diff {
                    min = diff;
                    result = sum;
                }
                if sum > target {
                    i2 -= 1;
                } else {
                    i1 += 1;
                }
            }
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::three_sum_closest(vec![-1,2,1,-4], 1));
}

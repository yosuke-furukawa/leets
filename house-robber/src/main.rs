impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            _ => {
                let mut memo = vec![nums[0], nums[0].max(nums[1])];
                for i in 2..nums.len() {
                    memo.push(memo[i - 1].max(memo[i - 2] + nums[i]));
                }
                *memo.last().unwrap()
            }
        }
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::rob(vec![1, 2, 3, 1]));
}

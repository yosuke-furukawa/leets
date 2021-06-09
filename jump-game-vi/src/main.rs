use std::collections::VecDeque;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let mut score = 0;
        let mut dp: VecDeque<(i32, i32)> = VecDeque::new();

        for (i, n) in nums.iter().enumerate() {
            if !dp.is_empty() && dp.front().unwrap().0 == i as i32 - k - 1 {
                dp.pop_front();
            }
            score = if !dp.is_empty() {
                dp.front().unwrap().1 + n
            } else {
                *n
            };
            while !dp.is_empty() && dp.back().unwrap().1 <= score {
                dp.pop_back();
            }
            dp.push_back((i as i32, score));
        }
        score
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2));
    println!("{}", Solution::max_result(vec![10, -5, -2, 4, 0, 3], 3));
}

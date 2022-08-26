use std::collections::HashSet;

impl Solution {
    fn dfs(cur_nums: &[i32], nums: &[i32], res: &mut HashSet<Vec<i32>>) {
        if cur_nums.len() >= 2 && !res.contains(cur_nums) {
            res.insert(cur_nums.to_vec());
        }
        for i in 0..nums.len() {
            if cur_nums.is_empty() || !cur_nums.is_empty() && cur_nums.last().unwrap() <= &nums[i] {
                let mut new_nums = cur_nums.to_vec();
                new_nums.push(nums[i]);
                Self::dfs(&new_nums, &nums[(i + 1)..], res);
            }
        }
    }
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = HashSet::new();
        Self::dfs(&[], &nums, &mut res);
        res.into_iter().collect()
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::find_subsequences(vec![4, 6, 7, 7]));
}

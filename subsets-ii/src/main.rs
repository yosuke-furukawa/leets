use std::collections::HashSet;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let size = 2_i32.pow(nums.len() as u32);
        let mut added = HashSet::new();
        let mut answer = vec![];
        for i in 0..size {
            let f = format!("{:b}", i);
            let mut temp = vec![];
            for (i, c) in f.chars().rev().enumerate() {
                if c == '1' {
                    temp.push(nums[i]);
                }
            }
            if added.contains(&temp) {
                continue;
            }
            added.insert(temp.clone());
            answer.push(temp);
        }
        answer
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::subsets_with_dup(vec![1, 2, 2]));
    println!("{:?}", Solution::subsets_with_dup(vec![4, 4, 4, 1, 4]));
}

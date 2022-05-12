use std::collections::HashSet;

impl Solution {
    fn permute(num: &mut [i32], k: usize, res: &mut HashSet<Vec<i32>>) {
        for i in k..num.len() {
            num.swap(i, k);
            Self::permute(num, k + 1, res);
            num.swap(k, i);
        }
        res.insert(num.to_vec());
    }
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut res = HashSet::new();
        Self::permute(&mut nums, 0, &mut res);
        res.into_iter().collect()
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::permute_unique(vec![1, 1, 2]));
}

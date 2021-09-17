use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut counter = HashMap::new();
        for n in nums1 {
            *counter.entry(n).or_insert(0) += 1;
        }
        let mut res = vec![];
        for n in nums2.into_iter() {
            if let Some(v) = counter.get_mut(&n) {
                if *v > 0 {
                    *v -= 1;
                    res.push(n);
                }
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]));
}

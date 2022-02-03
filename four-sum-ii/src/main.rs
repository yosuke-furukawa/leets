use std::collections::HashMap;

impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for n1 in nums1.iter() {
            for n2 in nums2.iter() {
                *map.entry(n1 + n2).or_insert(0) += 1;
            }
        }
        let mut count = 0;
        for n3 in nums3.iter() {
            for n4 in nums4.iter() {
                if let Some(v) = map.get(&(0 - n3 - n4)) {
                    count += v;
                }
            }
        }
        count
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]));
}

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut stack = vec![];
        let mut greater = vec![-1; nums2.len()];
        for (i, n) in nums2.iter().enumerate() {
            while !stack.is_empty() && *n > nums2[*stack.last().unwrap() as usize] {
                let i = stack.pop().unwrap() as usize;
                greater[i] = *n;
            }
            stack.push(i);
            map.insert(n, i);
        }

        nums1
            .iter()
            .map(|x| greater[*map.get(x).unwrap()])
            .collect()
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
    );
    println!(
        "{:?}",
        Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4])
    );
}

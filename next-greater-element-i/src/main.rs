use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, n1) in nums2.iter().enumerate() {
            map.insert(n1, -1);
            for n2 in nums2.iter().skip(i + 1) {
                if n2 > n1 {
                    map.insert(n1, *n2);
                    break;
                }
            }
        }

        nums1.iter().map(|x| *map.get(x).unwrap()).collect()
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
        Solution::next_greater_element(vec![2,4], vec![1, 2, 3, 4])
    );
}

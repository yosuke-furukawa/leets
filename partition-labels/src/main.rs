use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut res = vec![];
        let mut last = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            *last.entry(c).or_insert(i as i32) = i as i32;
        }
        let mut start = 0;
        let mut end = 0;
        for (i, c) in s.chars().enumerate() {
            end = std::cmp::max(end, *last.get(&c).unwrap());
            if end == i as i32 {
                res.push(end - start + 1);
                start = end + 1;
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::partition_labels("ababcbacadefegdehijhklij".to_string())
    );
}

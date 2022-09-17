use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        let mut firsts = HashMap::new();
        let mut lasts = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            *counts.entry(num).or_insert(0) += 1;
            firsts.entry(num).or_insert(i);
            lasts.insert(num, i);
        }
        let max_count = counts.values().max().unwrap();
        let entries = counts
            .iter()
            .filter(|&(_, count)| count == max_count)
            .collect::<Vec<_>>();
        let mut result = std::i32::MAX;
        for (key, _) in entries {
            let first = *firsts.get(key).unwrap() as i32;
            let last = *lasts.get(key).unwrap() as i32;
            result = result.min(last - first + 1);
        }
        result as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]));
    println!(
        "{}",
        Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2])
    );
}

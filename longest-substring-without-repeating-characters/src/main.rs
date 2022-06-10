use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut res = 0;
        let mut j = 0;
        for (i, c) in s.chars().enumerate() {
            if let Some(temp) = map.get(&c) {
                j = j.max(*temp + 1);
            }
            res = res.max(i - j + 1);
            map.insert(c, i);
        }
        res as i32
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::length_of_longest_substring("abcabcbb".to_string())
    );
}

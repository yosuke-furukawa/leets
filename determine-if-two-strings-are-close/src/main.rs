use std::collections::HashMap;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut word1_map: HashMap<char, i32> = HashMap::new();
        let mut word2_map: HashMap<char, i32> = HashMap::new();
        for c in word1.chars() {
            *word1_map.entry(c).or_insert(0) += 1;
        }

        for c in word2.chars() {
            *word2_map.entry(c).or_insert(0) += 1;
        }

        let mut key1: Vec<char> = word1_map.keys().cloned().collect();
        let mut key2: Vec<char> = word2_map.keys().cloned().collect();
        key1.sort();
        key2.sort();
        if key1 != key2 {
            return false;
        }

        let mut val1: Vec<i32> = word1_map.values().cloned().collect();
        let mut val2: Vec<i32> = word2_map.values().cloned().collect();
        val1.sort();
        val2.sort();

        val1 == val2
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::close_strings("test".to_string(), "tset".to_string())
    );
}

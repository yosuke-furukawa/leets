use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut key_to_count: HashMap<String, i32> = HashMap::new();
        let mut count_to_keys: HashMap<i32, HashSet<String>> = HashMap::new();
        let mut max_count = 0;
        for word in words.iter() {
            let count = key_to_count.entry(word.clone()).or_insert(0);
            if let Some(keys) = count_to_keys.get_mut(count) {
                keys.remove(word);
            }
            *count += 1;
            max_count = max_count.max(*count);
            count_to_keys
                .entry(*count)
                .or_insert_with(HashSet::new)
                .insert(word.clone());
        }
        let mut res = vec![];
        for i in (1..=max_count).rev() {
            if let Some(keys) = count_to_keys.get(&i) {
                let mut keys: Vec<String> = keys.iter().cloned().collect();
                keys.sort_unstable();
                for key in keys.iter() {
                    res.push(key.clone());
                    if res.len() == k as usize {
                        return res;
                    }
                }
            }
        }
        res
    }
}

struct Solution;

fn stringify(s: Vec<&str>) -> Vec<String> {
    s.iter().map(|&s| s.to_string()).collect()
}

fn main() {
    println!(
        "{:?}",
        Solution::top_k_frequent(
            stringify(vec!["i", "love", "leetcode", "i", "love", "coding"]),
            2
        )
    );
}

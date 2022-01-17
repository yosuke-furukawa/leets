use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let chars: Vec<char> = pattern.chars().collect();
        let words: Vec<String> = s.split_ascii_whitespace().map(|x| x.to_string()).collect();
        let mut map: HashMap<char, String> = HashMap::new();
        let mut set: HashSet<String> = HashSet::new();
        if chars.len() != words.len() {
            return false;
        }

        for (i, c) in chars.iter().enumerate() {
            let word = &words[i];
            if let Some(w) = map.get(c) {
                if *w != *word {
                    return false;
                }
            } else {
                if set.contains(word) {
                    return false;
                }
                map.insert(*c, word.clone());
                set.insert(word.clone());
            }
        }
        true
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string())
    );
}

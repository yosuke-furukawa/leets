use std::collections::HashSet;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut set = HashSet::new();
        for c in sentence.chars() {
            set.insert(c);
        }
        set.len() == 26
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_string())
    );
}

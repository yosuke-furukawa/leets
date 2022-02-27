use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = HashMap::new();
        for c in magazine.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        for c in ransom_note.chars() {
            if !map.contains_key(&c) {
                return false;
            }
            let count = map.entry(c).or_insert(0);
            if *count == 0 {
                return false;
            }
            *count -= 1;
        }
        true
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::can_construct("a".to_string(), "b".to_string())
    );
}

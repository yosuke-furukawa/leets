use std::collections::HashMap;

impl Solution {
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        let mut map = HashMap::new();
        for (i, v) in keyboard.chars().enumerate() {
            map.insert(v, i as i32);
        }
        let mut current = 0;
        let mut result = 0;
        for c in word.chars() {
            let next = *map.get(&c).unwrap();
            let pos = (current - next).abs();
            result += pos;
            current = next;
        }
        result
    }
}
struct Solution;

fn main() {
    println!(
        "{}",
        Solution::calculate_time("abcdefghijklmnopqrstuvwxyz".to_string(), "cba".to_string())
    );
}

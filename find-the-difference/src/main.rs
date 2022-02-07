use std::collections::HashMap;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut map = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            if let Some(v) = map.get_mut(&c) {
                if *v == 0 {
                    return c;
                } else {
                    *v -= 1;
                }
            } else {
                return c;
            }
        }
        ' '
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::find_the_difference("abcd".to_string(), "abcde".to_string())
    );
}

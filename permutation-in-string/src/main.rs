use std::collections::HashMap;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut map = HashMap::new();
        for c in s1.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        let s2_chars: Vec<char> = s2.chars().collect();
        for pairs in s2_chars.windows(s1.len()) {
            let mut map_clone = map.clone();
            for c in pairs {
                *map_clone.entry(*c).or_insert(0) -= 1;
                if *map_clone.get(&*c).unwrap() < 0 {
                    break;
                }
            }
            if map_clone.values().all(|&x| x == 0) {
                return true;
            }
        }
        false
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()));
    println!("{}", Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()));
}

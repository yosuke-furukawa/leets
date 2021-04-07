use std::collections::HashMap;

impl Solution {
    fn create_map(chars: &[char], start: usize, end: usize) -> HashMap<String, usize> {
        let mut map: HashMap<String, usize> = HashMap::new();
        for c in chars.iter().take(end).skip(start) {
            match c {
                'a' | 'i' | 'u' | 'e' | 'o' | 'A' | 'I' | 'U' | 'E' | 'O' => {
                    *map.entry("vowels".to_string()).or_insert(0) += 1
                }
                _ => *map.entry("consonant".to_string()).or_insert(0) += 1,
            }
        }
        map
    }
    pub fn halves_are_alike(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }
        let half = s.len() / 2;
        let chars: Vec<char> = s.chars().collect();
        let first_half = Self::create_map(&chars, 0, half);
        let second_half = Self::create_map(&chars, half, s.len());

        first_half == second_half
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::halves_are_alike("book".to_string()));
}

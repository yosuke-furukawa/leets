use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut frequency: HashMap<char, usize> = HashMap::new();
        let (mut l, mut res) = (0, 0);

        for r in 0..s.len() {
            *frequency.entry(s[r]).or_default() += 1;
            let max_frequency = frequency.values().max().unwrap();
            let max_replacement = (r - l + 1) - max_frequency;
            if max_replacement > k as usize {
                *frequency.entry(s[l]).or_default() -= 1;
                l += 1;
            }
            res = usize::max(res, r - l + 1);
        }
        res as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::character_replacement("ABAB".to_string(), 2));
}

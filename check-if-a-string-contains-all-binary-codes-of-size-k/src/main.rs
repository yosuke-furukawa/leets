use std::collections::HashSet;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let len = 2i32.pow(k as u32) as usize;

        let mut set = HashSet::new();
        for ch in s
            .chars()
            .collect::<Vec<char>>()
            .as_slice()
            .windows(k as usize)
        {
            let st: String = ch.iter().collect();
            set.insert(st);
        }
        set.len() >= len
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::has_all_codes("00110110".to_string(), 2));
    println!("{}", Solution::has_all_codes("0110".to_string(), 1));
    println!("{}", Solution::has_all_codes("0110".to_string(), 2));
}

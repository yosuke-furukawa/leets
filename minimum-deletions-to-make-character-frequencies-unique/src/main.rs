use std::collections::HashSet;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut freqs = [0; 26];
        let mut res = 0;
        let mut set = HashSet::new();
        for c in s.chars() {
            freqs[(c as u8 - b'a') as usize] += 1;
        }
        for freq in &mut freqs {
            while *freq > 0 && !set.insert(*freq) {
                *freq -= 1;
                res += 1;
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_deletions("aaabbbcc".to_string()));
    println!(
        "{}",
        Solution::min_deletions("accdcdadddbaadbc".to_string())
    );
    println!("{}", Solution::min_deletions("ceabaacb".to_string()));
    println!("{}", Solution::min_deletions("voniaf".to_string()));
}

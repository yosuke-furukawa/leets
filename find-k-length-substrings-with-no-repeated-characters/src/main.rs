use std::collections::HashSet;

impl Solution {
    pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut result = 0;

        for w in chars.windows(k as usize) {
            let set: HashSet<char> = w.iter().cloned().collect();
            if set.len() == k as usize {
                result += 1;
            }
        }
        result
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::num_k_len_substr_no_repeats("havefunonleetcode".to_string(), 5)
    );

    println!(
        "{}",
        Solution::num_k_len_substr_no_repeats("home".to_string(), 5)
    );
}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = 0;
        while j < chars.len() {
            if chars[j] == ' ' {
                chars[i..j].reverse();
                i = j + 1;
            }
            j += 1;
        }
        chars[i..j].reverse();
        chars.iter().collect()
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::reverse_words("Let's take LeetCode contest".to_string())
    );
}

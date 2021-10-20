impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::reverse_words("the sky is blue".to_string()));
    println!("{}", Solution::reverse_words("  hello world  ".to_string()));
}

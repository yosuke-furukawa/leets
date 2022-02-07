impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        (s + &t).chars().fold(0, |sum, c| sum ^ c as u8) as char
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::find_the_difference("abcd".to_string(), "abcde".to_string())
    );
}

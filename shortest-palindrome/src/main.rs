impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let n = s.len();
        let rev = s.chars().rev().collect::<String>();
        for i in 0..n {
            if s.starts_with(&rev[i..]) {
                return rev + &s[n - i..];
            }
        }
        "".to_string()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::shortest_palindrome("abcd".to_string()));
    println!("{}", Solution::shortest_palindrome("aacecaaa".to_string()));
}

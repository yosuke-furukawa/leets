impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let len = palindrome.len();
        if len == 1 {
            return "".to_string();
        }
        let mut chars: Vec<char> = palindrome.chars().collect();
        let mut changed = false;
        for (i, c) in palindrome.chars().enumerate() {
            if i < len / 2 && c != 'a' {
                changed = true;
                chars[i] = 'a';
                break;
            }
        }
        if !changed {
            chars[len - 1] = 'b';
        }
        chars
            .iter()
            .fold(String::new(), |acc, cur| acc + &cur.to_string())
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::break_palindrome("abccba".to_string()));
    println!("{}", Solution::break_palindrome("a".to_string()));
    println!("{}", Solution::break_palindrome("aa".to_string()));
    println!("{}", Solution::break_palindrome("aba".to_string()));
}

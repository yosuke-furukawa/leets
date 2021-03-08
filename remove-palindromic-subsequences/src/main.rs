impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let half = s.len() / 2;
        for i in 0..half {
            let end = s.len() - i - 1;
            if s.get(i..i + 1) != s.get(end..end + 1) {
                return 2;
            }
        }
        1
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::remove_palindrome_sub("abbabba".to_string()));
    println!(
        "{}",
        Solution::remove_palindrome_sub("abbabbaaaa".to_string())
    );
    println!("{}", Solution::remove_palindrome_sub("".to_string()));
}

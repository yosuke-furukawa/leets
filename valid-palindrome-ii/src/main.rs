impl Solution {
    fn is_palindrome(s: &[u8], i: usize, j: usize) -> bool {
        if i >= j {
            return true;
        }
        if s[i] != s[j] {
            return false;
        }
        Self::is_palindrome(s, i + 1, j - 1)
    }
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            if s[left] != s[right] {
                return Self::is_palindrome(s, left + 1, right)
                    || Self::is_palindrome(s, left, right - 1);
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::valid_palindrome("aba".to_string()));
    println!("{}", Solution::valid_palindrome("abca".to_string()));
    println!("{}", Solution::valid_palindrome("abc".to_string()));
}

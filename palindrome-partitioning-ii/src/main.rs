use std::collections::HashMap;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();
        let mut memo = HashMap::new();
        Self::dp(0, n, &mut memo, &chars)
    }
    fn dp(
        start: usize,
        end: usize,
        memo: &mut HashMap<(usize, usize), i32>,
        chars: &[char],
    ) -> i32 {
        if let Some(&res) = memo.get(&(start, end)) {
            return res;
        }

        let res = if Self::is_palindrome(start, end, chars) {
            0
        } else {
            let mut res = std::i32::MAX;

            for i in start + 1..end {
                if Self::is_palindrome(start, i, chars) {
                    let next = Self::dp(i, end, memo, chars) + 1;
                    res = res.min(next);
                }
            }
            res
        };
        memo.insert((start, end), res);
        res
    }
    fn is_palindrome(start: usize, end: usize, s: &[char]) -> bool {
        let reversed: Vec<&char> = s[start..end].iter().rev().collect();
        for (i, c) in s[start..end].iter().enumerate() {
            if reversed[i] != c {
                return false;
            }
        }
        true
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_cut("aab".to_string()));
}

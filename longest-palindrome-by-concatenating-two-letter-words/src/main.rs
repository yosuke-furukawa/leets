use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut same_letters = HashMap::new();
        let mut diff_letters = HashMap::new();
        for word in words {
            let chars = word.chars().collect::<Vec<char>>();
            if chars[0] == chars[1] {
                let count = same_letters.entry(chars[0]).or_insert(0);
                *count += 1;
            } else {
                let count = diff_letters.entry((chars[0], chars[1])).or_insert(0);
                *count += 1;
            }
        }
        let mut result = 0;
        let mut mid = 0;
        for (_, count) in same_letters {
            result += count / 2 * 2;
            if count % 2 == 1 {
                mid = 1;
            }
        }
        for (&(c1, c2), count1) in &diff_letters {
            if let Some(count2) = diff_letters.get(&(c2, c1)) {
                result += count1.min(count2);
            }
        }
        (result + mid) * 2
    }
}

struct Solution;

fn stringify(s: Vec<&str>) -> Vec<String> {
    s.iter().map(|x| x.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::longest_palindrome(stringify(vec!["lc", "cl", "gg"]))
    );
    println!(
        "{}",
        Solution::longest_palindrome(stringify(vec!["ab", "ty", "yt", "lc", "cl", "ab"]))
    );
    println!(
        "{}",
        Solution::longest_palindrome(stringify(vec!["cc", "ll", "xx"]))
    );
}

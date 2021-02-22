use std::cmp::Ordering;

impl Solution {
    pub fn find_longest_word(s: String, d: Vec<String>) -> String {
        let mut pos: Vec<usize> = vec![0; d.len()];
        for c in s.chars() {
            for (i, p) in pos.iter_mut().enumerate() {
                let word = &d[i];
                if let Some(w) = word.chars().nth(*p) {
                    if w == c {
                        *p += 1;
                    }
                }
            }
        }
        let mut ans = "".to_string();
        for (i, p) in pos.iter().enumerate() {
            if d[i].len() == *p {
                let word = &d[i];
                if ans.len() < word.len()
                    || ans.len() == word.len() && word.cmp(&ans) == Ordering::Less
                {
                    ans = word.clone();
                }
            }
        }
        ans
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::find_longest_word(
            "abpcplea".to_string(),
            vec![
                "ale".to_string(),
                "apple".to_string(),
                "monkey".to_string(),
                "plea".to_string()
            ]
        )
    );
    println!(
        "{}",
        Solution::find_longest_word(
            "abpcplea".to_string(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        )
    );
    println!(
        "{}",
        Solution::find_longest_word(
            "bab".to_string(),
            vec![
                "ba".to_string(),
                "ab".to_string(),
                "a".to_string(),
                "b".to_string()
            ]
        )
    );
}

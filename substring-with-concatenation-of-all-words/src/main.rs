use std::collections::HashMap;

impl Solution {
    fn check(
        word_count: &HashMap<String, i32>,
        i: usize,
        s: &str,
        word_len: usize,
        substr_size: usize,
        k: usize,
    ) -> bool {
        let mut word_count = word_count.clone();
        let mut words_used = 0;
        let mut j = i;
        while j < i + substr_size {
            if s.len() < j + word_len {
                break;
            }
            let sub = &s[j..j + word_len];
            if let Some(&count) = word_count.get(sub) {
                if count == 0 {
                    break;
                }
                word_count.insert(sub.to_string(), count - 1);
                words_used += 1;
            } else {
                break;
            }
            j += word_len;
        }
        words_used == k
    }
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let n = s.len();
        let k = words.len();
        let word_len = words[0].len();
        let substr_size = word_len * k;
        let mut word_count = HashMap::<String, i32>::new();
        for word in words {
            *word_count.entry(word).or_insert(0) += 1;
        }
        let mut result = vec![];
        if n < substr_size {
            return result;
        }
        for i in 0..=n - substr_size {
            if Self::check(&word_count, i, &s, word_len, substr_size, k) {
                result.push(i as i32);
            }
        }
        result
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::find_substring(
            "barfoothefoobarman".to_string(),
            vec!["foo".to_string(), "bar".to_string()]
        )
    );
    println!(
        "{:?}",
        Solution::find_substring(
            "wordgoodgoodgoodbestword".to_string(),
            vec![
                "word".to_string(),
                "good".to_string(),
                "best".to_string(),
                "word".to_string()
            ]
        )
    );
    println!(
        "{:?}",
        Solution::find_substring("a".to_string(), vec!["a".to_string(), "a".to_string()])
    );
}

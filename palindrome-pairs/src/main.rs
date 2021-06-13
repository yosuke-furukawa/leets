use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        let mut set = HashSet::new();
        let n = words.len();

        if n == 0 {
            return vec![];
        }

        for (i, word) in words.clone().into_iter().enumerate() {
            map.insert(word, i);
        }

        for (i, word) in words.into_iter().enumerate() {
            let word_len = word.len();
            let mut prefix: String = String::new();
            let mut suffix = word.clone();
            let mut reversed_prefix: String = String::new();
            let mut reversed_suffix: String = suffix.clone().chars().rev().collect();

            for j in 0..word_len + 1 {
                if j > 0 {
                    prefix += word.get(j - 1..j).unwrap_or("");
                    suffix = suffix.get(1..).unwrap().to_string();
                    reversed_prefix =
                        word.get(j - 1..j).unwrap().to_string() + reversed_prefix.as_str();
                    reversed_suffix = reversed_suffix
                        .get(0..reversed_suffix.len() - 1)
                        .unwrap()
                        .to_string();
                }
                if prefix == reversed_prefix {
                    if let Some(&k) = map.get(&reversed_suffix) {
                        if k != i {
                            set.insert(vec![k as i32, i as i32]);
                        }
                    }
                }

                if suffix == reversed_suffix {
                    if let Some(&l) = map.get(&reversed_prefix) {
                        if l != i {
                            set.insert(vec![i as i32, l as i32]);
                        }
                    }
                }
            }
        }

        set.into_iter().collect()
    }
}

struct Solution;

fn stringify(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{:?}",
        Solution::palindrome_pairs(stringify(vec!["abcd", "dcba", "lls", "s", "sssll"]))
    );
}

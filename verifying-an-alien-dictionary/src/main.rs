use std::collections::HashMap;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

        let mut map = HashMap::new();
        for (i, c) in order.chars().enumerate() {
            map.insert(c, alphabet[i]);
        }

        let mut new_words: Vec<(usize, Vec<char>)> = words
            .iter()
            .enumerate()
            .map(|(i, w)| {
                (
                    i,
                    w.chars()
                        .map(|c| *map.get(&c).unwrap())
                        .collect::<Vec<char>>(),
                )
            })
            .collect();
        new_words.sort_unstable_by(|a, b| a.1.cmp(&b.1));

        for (i, new_word) in new_words.iter().enumerate() {
            if new_word.0 != i {
                return false;
            }
        }
        true
    }
}

struct Solution;

fn stringify(words: Vec<&str>) -> Vec<String> {
    words.iter().map(|x| x.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::is_alien_sorted(
            stringify(vec!["hello", "leetcode"]),
            "hlabcdefgijkmnopqrstuvwxyz".to_string()
        )
    );
    println!(
        "{}",
        Solution::is_alien_sorted(
            stringify(vec!["word", "world", "row"]),
            "worldabcefghijkmnpqstuvxyz".to_string()
        )
    );
    println!(
        "{}",
        Solution::is_alien_sorted(
            stringify(vec!["apple", "app"]),
            "abcdefghijklmnopqrstuvwxyz".to_string()
        )
    );
}

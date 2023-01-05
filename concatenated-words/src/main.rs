use std::collections::HashSet;

impl Solution {
    fn helper(word: &str, set: &HashSet<String>) -> bool {
        if word.is_empty() {
            return false;
        }
        let mut dp = vec![false; word.len() + 1];
        dp[0] = true;
        for i in 1..=word.len() {
            for j in 0..i {
                if dp[j] && set.contains(&word[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[word.len()]
    }
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let mut words = words;
        words.sort_unstable_by_key(|s| s.len());
        let mut set = HashSet::new();
        let mut res = vec![];
        for word in words {
            if Self::helper(&word, &set) {
                res.push(word.clone());
            }
            set.insert(word);
        }
        res
    }
}

struct Solution;

fn stringify(s: Vec<&str>) -> Vec<String> {
    s.into_iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{:?}",
        Solution::find_all_concatenated_words_in_a_dict(stringify(vec![
            "cat",
            "cats",
            "catsdogcats",
            "dog",
            "dogcatsdog",
            "hippopotamuses",
            "rat",
            "ratcatdogcat"
        ]))
    );
}

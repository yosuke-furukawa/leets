use std::collections::HashSet;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let set: HashSet<char> = s.chars().collect();
        let mut cache: HashSet<String> = HashSet::new();
        let mut count = 0;
        for word in words {
            let mut si = 0;
            let mut matched = 0;
            if cache.contains(&word) {
                count += 1;
                continue;
            }
            for w1 in word.chars() {
                if !set.contains(&w1) {
                    break;
                }
                for w2 in s.chars().skip(si) {
                    // println!("si ={}, i = {} w1 = {}, w2 = {}", si, i, w1, w2);
                    if w1 == w2 {
                        si += 1;
                        matched += 1;
                        break;
                    }
                    si += 1;
                }

                if matched == word.len() {
                    cache.insert(word.clone());
                    count += 1;
                }
            }
        }
        count
    }
}

struct Solution;

fn stringify(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::num_matching_subseq(
            "abcde".to_string(),
            stringify(vec!["a", "bb", "acd", "ace"])
        )
    );
    println!(
        "{}",
        Solution::num_matching_subseq(
            "dsahjpjauf".to_string(),
            stringify(vec!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"])
        )
    );
}

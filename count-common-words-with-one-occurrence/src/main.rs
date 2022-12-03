use std::collections::HashMap;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut map1 = HashMap::new();
        for word in words1 {
            *map1.entry(word).or_insert(0) += 1;
        }
        let mut map2 = HashMap::new();
        for word in words2 {
            *map2.entry(word).or_insert(0) += 1;
        }
        let mut res = 0;
        for (word, count) in map1 {
            if count == 1 && map2.get(&word).unwrap_or(&0) == &1 {
                res += 1;
            }
        }
        res
    }
}

struct Solution;

fn stringify(s: Vec<&str>) -> Vec<String> {
    s.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::count_words(
            stringify(vec!["leetcode", "is", "amazing", "as", "is"]),
            stringify(vec!["amazing", "leetcode", "is"])
        )
    );
}

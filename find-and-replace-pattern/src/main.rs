use std::collections::HashMap;

impl Solution {
    fn create_pattern(word: String) -> Vec<i32> {
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut num = 0;
        let mut pattern = vec![];
        for c in word.chars() {
            if let Some(v) = map.get(&c) {
                pattern.push(*v);
            } else {
                map.insert(c, num);
                pattern.push(num);
                num += 1;
            }
        }
        pattern
    }
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let plen = pattern.len();
        let p = Self::create_pattern(pattern);
        let mut results = vec![];
        for word in words {
            if word.len() != plen {
                continue;
            }
            let p2 = Self::create_pattern(word.clone());
            if p == p2 {
                results.push(word);
            }
        }
        results
    }
}

struct Solution;

fn stringify(words: Vec<&str>) -> Vec<String> {
    words.into_iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!("{:?}", Solution::find_and_replace_pattern(stringify(vec!["abc","deq","mee","aqq","dkd","ccc"]
), "abb".to_string()));
}

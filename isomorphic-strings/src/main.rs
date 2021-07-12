use std::collections::HashMap;

impl Solution {
    fn convert_vec(s: String) -> Vec<usize> {
        let mut map = HashMap::new();
        let mut result = vec![];
        for (i, c) in s.chars().enumerate() {
            let v = map.entry(c).or_insert(i);
            result.push(*v);
        }
        result
    }
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        Solution::convert_vec(s) == Solution::convert_vec(t)
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::is_isomorphic("egg".to_string(), "add".to_string())
    );
    println!(
        "{}",
        Solution::is_isomorphic("foo".to_string(), "bar".to_string())
    );
    println!(
        "{}",
        Solution::is_isomorphic("paper".to_string(), "title".to_string())
    );
}

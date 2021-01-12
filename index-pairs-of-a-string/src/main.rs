struct Solution {}
impl Solution {
    pub fn index_pairs(text: String, words: Vec<String>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        for offset in 0..text.len() {
            for word in &words {
                let t = text.get(offset..text.len());
                if t.unwrap().starts_with(word) {
                    let start = offset as i32;
                    let end = (offset + word.len()) as i32 - 1;
                    result.push(vec![start, end]);
                }
            }
        }
        result.sort_unstable();
        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::index_pairs(
            "thestoryofleetcodeandme".to_string(),
            vec!["story", "fleet", "leetcode"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        )
    );
    println!(
        "{:?}",
        Solution::index_pairs(
            "ababa".to_string(),
            vec!["aba", "ab"].iter().map(|s| s.to_string()).collect()
        )
    );
}

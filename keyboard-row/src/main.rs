impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let rows = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];

        words
            .into_iter()
            .filter(|s| {
                rows.iter()
                    .any(|row| s.to_lowercase().chars().all(|c| row.contains(c)))
            })
            .collect()
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::find_words(vec![
            "Hello".to_string(),
            "Alaska".to_string(),
            "Dad".to_string(),
            "Peace".to_string()
        ])
    );
}

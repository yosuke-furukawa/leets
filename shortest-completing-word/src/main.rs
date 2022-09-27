impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut plate = [0; 26];
        for c in license_plate.chars() {
            if c.is_ascii_alphabetic() {
                plate[c.to_ascii_lowercase() as usize - 'a' as usize] += 1;
            }
        }
        let mut ans = String::new();
        for word in words {
            let mut word_count = [0; 26];
            for c in word.chars() {
                word_count[c as usize - 'a' as usize] += 1;
            }
            if word_count.iter().zip(plate.iter()).all(|(&a, &b)| a >= b) && (ans.is_empty()
                || word.len() < ans.len())
            {
                ans = word;
            }
        }
        ans
    }
}

struct Solution;

fn stringify(s: Vec<&str>) -> Vec<String> {
    s.iter().map(|&s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::shortest_completing_word(
            "1s3 PSt".to_string(),
            stringify(vec!["step", "steps", "stripe", "stepple"])
        )
    );
    println!(
        "{}",
        Solution::shortest_completing_word(
            "1s3 456".to_string(),
            stringify(vec!["looks", "pest", "stew", "show"])
        )
    );
}

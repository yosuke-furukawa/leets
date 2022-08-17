use std::collections::HashSet;

const MORSE: &[&str] = &[
    ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
    "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
];

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let mut set = HashSet::new();
        for word in words {
            let mut morse = String::new();
            for c in word.chars() {
                morse.push_str(MORSE[(c as u8 - b'a') as usize]);
            }
            set.insert(morse);
        }
        set.len() as i32
    }
}

struct Solution;

fn stringify(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::unique_morse_representations(stringify(vec!["gin", "zen", "gig", "msg"]))
    );
}

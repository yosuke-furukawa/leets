use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn replace_vowels(s: String) -> String {
        let mut result = s;
        result = result.replace("a", "*");
        result = result.replace("i", "*");
        result = result.replace("u", "*");
        result = result.replace("e", "*");
        result = result.replace("o", "*");
        result = result.replace("A", "*");
        result = result.replace("I", "*");
        result = result.replace("U", "*");
        result = result.replace("E", "*");
        result = result.replace("O", "*");
        result
    }
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let words: HashSet<String> = wordlist.iter().cloned().collect();
        let mut capitalized_suggests: HashMap<String, Vec<String>> = HashMap::new();
        let mut vowel_suggests: HashMap<String, Vec<String>> = HashMap::new();
        let mut result = vec![];

        for word in wordlist.iter() {
            let capital_key = word.to_ascii_uppercase();
            capitalized_suggests
                .entry(capital_key.clone())
                .or_default()
                .push(word.clone());
            let vowel_key = Self::replace_vowels(capital_key.clone());
            vowel_suggests
                .entry(vowel_key.clone())
                .or_default()
                .push(word.clone());
        }

        for query in queries {
            let key = Self::replace_vowels(query.clone()).to_ascii_uppercase();
            if words.contains(&query) {
                result.push(query);
            } else if let Some(s) = capitalized_suggests.get(&query.to_ascii_uppercase()) {
                result.push(s.first().unwrap().clone());
            } else if let Some(s) = vowel_suggests.get(&key) {
                result.push(s.first().unwrap().clone());
            } else {
                result.push("".to_string());
            }
        }

        result
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::spellchecker(
            vec![
                "KiTe".to_string(),
                "kite".to_string(),
                "hare".to_string(),
                "Hare".to_string()
            ],
            vec![
                "kite".to_string(),
                "Kite".to_string(),
                "KiTe".to_string(),
                "Hare".to_string(),
                "HARE".to_string(),
                "Hear".to_string(),
                "hear".to_string(),
                "keti".to_string(),
                "keet".to_string(),
                "keto".to_string()
            ]
        )
    );
}

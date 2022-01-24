impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word.to_lowercase() == word {
            return true;
        }
        if word.to_uppercase() == word {
            return true;
        }
        let chars: Vec<char> = word.chars().collect();
        if chars[0].is_uppercase() && chars[1..].iter().all(|&x| x.is_lowercase()) {
            return true;
        }
        false
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::detect_capital_use("USA".to_string()));
    println!("{}", Solution::detect_capital_use("leetcode".to_string()));
    println!("{}", Solution::detect_capital_use("Google".to_string()));
    println!("{}", Solution::detect_capital_use("FlaG".to_string()));
}

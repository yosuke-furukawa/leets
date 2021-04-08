use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let hm: HashMap<char, Vec<char>> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
        .iter()
        .map(|(d, v)| (*d, v.chars().collect()))
        .collect();

        if digits.is_empty() {
            return vec![];
        }
        let digits: Vec<char> = digits.chars().collect();
        let mut alphas = vec![];

        for n in digits {
            if let Some(chars) = hm.get(&n) {
                if alphas.is_empty() {
                    for c in chars {
                        alphas.push(c.to_string());
                    }
                } else {
                    let mut new_array = vec![];
                    for alpha in alphas.iter() {
                        for c in chars {
                            new_array.push(alpha.clone() + c.to_string().as_str());
                        }
                    }
                    alphas = new_array;
                }
            }
        }
        alphas
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::letter_combinations("23".to_string()));
}

use std::collections::BTreeSet;
use std::iter::FromIterator;

impl Solution {
    fn helper(s: String, charset: BTreeSet<char>) -> String {
        for c in charset.iter() {
            let mut charset = charset.clone();
            charset.remove(c);
            let s: String = (s.split_at(s.find(*c).unwrap()).1).to_string();
            if charset.is_subset(&BTreeSet::from_iter(s.chars())) {
                return format!("{}{}", c, Self::helper(s, charset));
            }
        }
        "".to_string()
    }
    pub fn remove_duplicate_letters(s: String) -> String {
        Self::helper(s.clone(), BTreeSet::from_iter(s.chars()))
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::remove_duplicate_letters("bcabc".to_string())
    );
}

use std::collections::HashMap;

impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        let mut map = HashMap::new();
        map.insert("6", "9");
        map.insert("9", "6");
        map.insert("8", "8");
        map.insert("1", "1");
        map.insert("0", "0");
        let half = num.len() / 2;
        if num.len() % 2 == 1 {
            let s = num.get(half..half + 1);
            if s != Some("8") && s != Some("1") && s != Some("0") {
                return false;
            }
        }
        for i in 0..half {
            let first = i;
            let last = num.len() - i - 1;
            let first_char = num.get(first..first + 1).unwrap();
            let last_char = num.get(last..last + 1).unwrap();
            if map.get(first_char) != Some(&last_char) {
                return false;
            }
        }
        true
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::is_strobogrammatic("69".to_string()));
    println!("{}", Solution::is_strobogrammatic("88".to_string()));
    println!("{}", Solution::is_strobogrammatic("962".to_string()));
    println!("{}", Solution::is_strobogrammatic("1".to_string()));
}

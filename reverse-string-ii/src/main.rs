impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let mut j = i + k as usize;
            if j > chars.len() {
                j = chars.len();
            }
            chars[i..j].reverse();
            i += 2 * k as usize;
        }
        chars.iter().collect()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::reverse_str("abcdefg".to_string(), 2));
    println!("{}", Solution::reverse_str("abcd".to_string(), 2));
}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() {
            return false;
        }
        let mut si = 0;
        for c in t.chars() {
            if si == s.len() {
                break;
            }
            if s.get(si..si + 1).unwrap() == c.to_string() {
                si += 1;
            }
        }
        si == s.len()
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string())
    );
}

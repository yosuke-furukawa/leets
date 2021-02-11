impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut vs = s.chars().collect::<Vec<char>>();
        let mut ts = t.chars().collect::<Vec<char>>();
        if vs.len() != ts.len() {
            return false;
        }
        vs.sort_unstable();
        ts.sort_unstable();
        vs == ts
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string())
    );
}

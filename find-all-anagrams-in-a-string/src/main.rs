impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut result = vec![];
        let mut p_set = vec![0; 26];
        let mut s_set = vec![0; 26];
        for c in p.chars() {
            p_set[(c as u8 - b'a') as usize] += 1;
        }
        for i in 0..s.len() {
            s_set[(s.as_bytes()[i] - b'a') as usize] += 1;
            if i >= p.len() {
                s_set[(s.as_bytes()[i - p.len()] - b'a') as usize] -= 1;
            }
            if s_set == p_set {
                result.push(i as i32 - p.len() as i32 + 1);
            }
        }
        result
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string())
    );
    println!(
        "{:?}",
        Solution::find_anagrams("abab".to_string(), "ab".to_string())
    );
}

use std::cmp::Reverse;

impl Solution {
    fn is_subsequence(a: String, b: String) -> bool {
        let mut it = a.chars().peekable();
        for c in b.chars() {
            if let Some(&first) = it.peek() {
                if first == c {
                    it.next();
                }
            }
        }
        it.next().is_none()
    }
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        if strs.len() == 1 {
            return -1;
        }
        let mut strs = strs;
        strs.sort_unstable_by_key(|b| Reverse(b.len()));

        for (i, s1) in strs.iter().enumerate() {
            let mut count = 0;
            for (j, s2) in strs.iter().enumerate() {
                if i != j && !Self::is_subsequence(s1.clone(), s2.clone()) {
                    count += 1;
                }
            }
            if count == strs.len() - 1 {
                return strs[i].len() as i32;
            }
        }
        -1
    }
}

struct Solution;

fn stringify(s: Vec<&str>) -> Vec<String> {
    s.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::find_lu_slength(stringify(vec!["aba", "cdc", "eae"]))
    );
}

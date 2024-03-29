use std::collections::HashMap;

const M: u64 = (1 << 31) - 1;
const B: u64 = 257;

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let s: &[u8] = s.as_bytes();
        let mut hashs: Vec<u64> = Vec::with_capacity(s.len() + 1);
        let mut hash = 0;
        hashs.push(hash);
        for &u in s.iter() {
            hash = (hash * B + u as u64) % M;
            hashs.push(hash);
        }

        let (mut l, mut r) = (0, s.len());
        let mut answer: usize = 0;
        while l < r {
            let m = l + (r - l) / 2;
            if let Some(a) = Solution::helper(s, m, &hashs) {
                answer = a;
                l = m + 1
            } else {
                r = m
            }
        }
        s[answer..answer + l - 1]
            .iter()
            .map(|&u| u as char)
            .collect()
    }
    fn helper(s: &[u8], len: usize, hashs: &[u64]) -> Option<usize> {
        let b = (0..len as u64).fold(1, |acc, _| (acc * B) % M);
        let mut hm: HashMap<u64, usize> = HashMap::new();
        for i in 0..s.len() - len + 1 {
            let h = (hashs[i + len] + M - hashs[i] * b % M) % M;
            if let Some(j) = hm.get(&h) {
                if (0..len).all(|k| s[i + k] == s[j + k]) {
                    return Some(*j);
                }
            } else {
                hm.insert(h, i);
            }
        }
        None
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::longest_dup_substring("banana".to_string()));
    println!("{}", Solution::longest_dup_substring("abcd".to_string()));
}

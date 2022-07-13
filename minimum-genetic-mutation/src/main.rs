use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let start: Vec<char> = start.chars().collect();
        let end: Vec<char> = end.chars().collect();
        let bank: Vec<Vec<char>> = bank.iter().map(|s| s.chars().collect()).collect();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start.clone(), 0));
        visited.insert(start);
        while !queue.is_empty() {
            let (item, step) = queue.pop_front().unwrap();
            if item == end {
                return step;
            }
            for candidate in bank.iter() {
                if !visited.contains(candidate) && Self::is_replaceable(&item, candidate) {
                    queue.push_back((candidate.clone(), step + 1));
                    visited.insert(candidate.clone());
                }
            }
        }
        -1
    }
    fn is_replaceable(a: &[char], b: &[char]) -> bool {
        let mut res = 0;
        for i in 0..a.len() {
            if a[i] != b[i] {
                res += 1;
            }
            if res > 1 {
                return false;
            }
        }
        res == 1
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::min_mutation(
            "AACCGGTT".to_string(),
            "AACCGGTA".to_string(),
            vec!["AACCGGTA".to_string()]
        )
    );
    println!(
        "{}",
        Solution::min_mutation(
            "AACCGGTT".to_string(),
            "AAACGGTA".to_string(),
            vec![
                "AACCGGTA".to_string(),
                "AACCGCTA".to_string(),
                "AAACGGTA".to_string()
            ]
        )
    );
    println!(
        "{}",
        Solution::min_mutation(
            "AAAAACCC".to_string(),
            "AACCCCCC".to_string(),
            vec![
                "AAAACCCC".to_string(),
                "AAACCCCC".to_string(),
                "AACCCCCC".to_string()
            ]
        )
    );
}

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    fn normalize(n: i32) -> i32 {
        match n {
            _ if n >= 10 => 0,
            _ if n < 0 => 9,
            _ => n,
        }
    }
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let deadends: HashSet<String> = deadends.into_iter().collect();
        if deadends.contains(&"0000".to_string()) {
            return -1;
        }
        let diffs = [
            [1, 0, 0, 0],
            [-1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, -1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, -1, 0],
            [0, 0, 0, 1],
            [0, 0, 0, -1],
        ];
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let init = "0000".to_string();
        visited.insert(init);
        queue.push_back(([0, 0, 0, 0], 0));
        while !queue.is_empty() {
            let k = queue.pop_front().unwrap();
            let key = k.0;
            let path = k.1;
            let keys = key.iter().fold(String::new(), |mut acc, n| {
                acc += n.to_string().as_str();
                acc
            });
            if keys == target {
                return path;
            }
            for diff in diffs.iter() {
                let mut new_array = [0; 4];
                for (i, d) in diff.iter().enumerate() {
                    new_array[i] = Self::normalize(key[i] + d);
                }
                let s = new_array.iter().fold(String::new(), |mut acc, n| {
                    acc += n.to_string().as_str();
                    acc
                });
                if !deadends.contains(&s) && !visited.contains(&s) {
                    visited.insert(s);
                    queue.push_back((new_array, path + 1));
                }
            }
        }
        -1
    }
}

struct Solution;

fn stringify(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::open_lock(
            stringify(vec!["0201", "0101", "0102", "1212", "2002"]),
            "0202".to_string()
        )
    );
}

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for n in arr {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut set = HashSet::new();
        for (_, v) in map {
            if !set.insert(v) {
                return false;
            }
        }
        true
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
}

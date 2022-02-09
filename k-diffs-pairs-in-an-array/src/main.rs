use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        for n in nums.iter() {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut set = HashSet::new();
        for n in nums.iter() {
            if set.contains(&(*n - k, n)) {
                continue;
            }
            if let Some(b) = map.get(&(*n - k)) {
                if k == 0 {
                    if *b > 1 {
                        set.insert((*n - k, n));
                    }
                } else {
                    set.insert((*n - k, n));
                }
            }
        }
        set.len() as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_pairs(vec![3, 1, 4, 1, 5], 2));
    println!("{}", Solution::find_pairs(vec![1, 2, 3, 4, 5], 1));
    println!("{}", Solution::find_pairs(vec![1, 3, 1, 5, 4], 0));
}

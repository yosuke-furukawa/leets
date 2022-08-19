use std::collections::HashMap;

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        let mut need: HashMap<i32, i32> = HashMap::new();
        for &n in nums.iter() {
            *freq.entry(n).or_insert(0) += 1;
        }

        for n in nums.iter() {
            if *freq.get(n).unwrap() == 0 {
                continue;
            } else if *need.get(n).unwrap_or(&0) > 0 {
                *need.entry(*n).or_default() -= 1;
                *need.entry(*n + 1).or_default() += 1;
            } else if *freq.get(&(n + 1)).unwrap_or(&0) > 0 && *freq.get(&(n + 2)).unwrap_or(&0) > 0
            {
                *freq.entry(n + 1).or_default() -= 1;
                *freq.entry(n + 2).or_default() -= 1;
                *need.entry(n + 3).or_default() += 1;
            } else {
                return false;
            }
            *freq.entry(*n).or_default() -= 1;
        }
        true
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::is_possible(vec![1, 2, 3, 3, 4, 5]));
    println!("{}", Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]));
}

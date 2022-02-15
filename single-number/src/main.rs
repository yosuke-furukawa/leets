use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for num in nums.iter() {
            if set.contains(num) {
                set.remove(num);
            } else {
                set.insert(*num);
            }
        }
        *set.iter().next().unwrap()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::single_number(vec![2, 2, 1]));
}

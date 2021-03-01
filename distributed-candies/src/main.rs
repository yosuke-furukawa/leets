use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let max_candy = (candy_type.len() / 2) as i32;
        let kinds = candy_type.into_iter().collect::<HashSet<i32>>().len() as i32;
        if max_candy < kinds {
            return max_candy;
        }
        kinds
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]));
}

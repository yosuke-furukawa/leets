use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut answer = vec![];
        for (i, n) in nums.iter().enumerate() {
            if let Some(j) = map.get(&(target - n)) {
                answer.push(*j as i32);
                answer.push(i as i32);
                break;
            }
            map.insert(n, i);
        }
        answer
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", Solution::two_sum(vec![3, 2, 4], 6));
    println!("{:?}", Solution::two_sum(vec![3, 3], 6));
}

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        let mut count = 0;
        let mut sum = 0;
        map.insert(0, 1);
        for n in nums.iter() {
            sum += n;
            if let Some(b) = map.get(&(sum - k)) {
                count += *b;
            }
            *map.entry(sum).or_insert(0) += 1;
        }
        count
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::subarray_sum(vec![1, 1, 1], 2));
    println!("{}", Solution::subarray_sum(vec![1, 2, 3], 3));
}

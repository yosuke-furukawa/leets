use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut count = 0;
        map.insert(0, -1);
        let mut max_len = 0;
        for (i, n) in nums.iter().enumerate() {
            count += if *n == 1 { 1 } else { -1 };
            if let Some(j) = map.get(&count) {
                max_len = max_len.max(i as i32 - j);
            } else {
                map.insert(count, i as i32);
            }
        }
        max_len
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_max_length(vec![0, 1]));
    println!("{}", Solution::find_max_length(vec![0, 1, 0]));
    println!("{}", Solution::find_max_length(vec![0, 1, 1, 0]));
    println!("{}", Solution::find_max_length(vec![0, 1, 1, 1, 0]));
}

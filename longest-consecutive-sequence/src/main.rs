use std::collections::HashMap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut max = 0;
        for num in nums {
            if !map.contains_key(&num) {
                let l = *map.get(&(num - 1)).unwrap_or(&0);
                let r = *map.get(&(num + 1)).unwrap_or(&0);
                let v = l + r + 1;
                max = max.max(v);
                map.insert(num, v);
                map.insert(num - l, v);
                map.insert(num + r, v);
            }
        }
        max
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2])
    );
    println!(
        "{}",
        Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
    );
}

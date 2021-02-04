use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }

        let mut max = 0;
        for (k, v) in map.iter() {
            let d = map.get(&(k + 1)).unwrap_or(&0);
            if d > &0 {
                max = max.max(v + d);
            }
        }
        max
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]));
}

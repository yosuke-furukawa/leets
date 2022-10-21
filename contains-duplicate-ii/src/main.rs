use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&num) {
                if i as i32 - j <= k {
                    return true;
                }
            }
            map.insert(num, i as i32);
        }
        false
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3)
    );
    println!(
        "{}",
        Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1)
    );
    println!(
        "{}",
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2)
    );
}

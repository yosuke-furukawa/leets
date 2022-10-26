use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::from([(0, -1)]);
        let mut partial_sum = vec![0; nums.len()];
        let mut sum = 0;

        for (i, n) in nums.iter().enumerate() {
            partial_sum[i] = (sum + n) % k;
            sum = partial_sum[i];
            if let Some(entry) = map.get(&sum) {
                if entry + 1 < i as i32 {
                    return true;
                }
                continue;
            }
            map.insert(sum, i as i32);
        }
        false
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
}

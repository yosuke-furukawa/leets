use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut max = 0;
        let mut sub = VecDeque::new();
        let mut map = HashMap::new();
        for num in nums {
            if sub.len() < k as usize {
                sub.push_back(num as i64);
                *map.entry(num as i64).or_insert(0) += 1;
            }
            if sub.len() == k as usize && map.len() == k as usize {
                let sum = sub.iter().sum::<i64>();
                max = max.max(sum);
            }
            if sub.len() == k as usize {
                let num = sub.pop_front().unwrap();
                if let Some(count) = map.get_mut(&num) {
                    *count -= 1;
                    if *count == 0 {
                        map.remove(&num);
                    }
                }
            }
        }
        max
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3)
    );
    println!(
        "{}",
        Solution::maximum_subarray_sum(vec![9, 9, 9, 1, 2, 3], 3)
    );
}

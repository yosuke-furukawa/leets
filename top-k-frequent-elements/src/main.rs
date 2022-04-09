use std::collections::BinaryHeap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut heap = BinaryHeap::new();
        for (key, value) in map {
            heap.push((-value, key));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        heap.into_sorted_vec().into_iter().map(|(_, k)| k).collect()
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2));
}

use std::collections::HashMap;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
        for n in arr.iter() {
            *counter.entry(n).or_insert(0) += 1;
        }
        let mut array: Vec<(&i32, i32)> = counter.into_iter().collect();
        array.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        let mut len = arr.len() as i32;
        let half = arr.len() as i32 / 2;
        let mut count = 0;
        while len > half {
            let item = array.pop().unwrap();
            len -= item.1;
            count += 1;
        }
        count
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7])
    );
    println!("{}", Solution::min_set_size(vec![7, 7, 7, 7, 7, 7]));
    println!("{}", Solution::min_set_size(vec![1, 9]));
    println!("{}", Solution::min_set_size(vec![1000, 1000, 3, 7]));
    println!(
        "{}",
        Solution::min_set_size(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    );
}

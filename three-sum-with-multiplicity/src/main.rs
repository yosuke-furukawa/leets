use std::collections::HashMap;

impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let mut array = arr;
        array.sort_unstable();
        let count_map = array.iter().enumerate().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c.1).or_insert_with(Vec::new).push(c.0);
            acc
        });
        let mut count = 0;
        for x in 0..array.len() {
            for y in x + 1..array.len() {
                let t = target - array[x] - array[y];
                if t < array[y] {
                    break;
                }
                if let Some(c) = count_map.get(&t) {
                    let first = c.first().unwrap();
                    let last = c.last().unwrap();
                    let f = if *first <= y { y } else { *first - 1 };
                    let co = if *last < f { 0 } else { *last - f };
                    count = (count + co) % 1_000_000_007;
                }
            }
        }
        count as i32
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8)
    );
    println!("{}", Solution::three_sum_multi(vec![0, 2, 0], 2));
}

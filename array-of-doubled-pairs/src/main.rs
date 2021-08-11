use std::collections::HashMap;

impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort_unstable_by_key(|a| a.abs());
        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in arr.iter() {
            *map.entry(*n).or_insert(0) += 1;
        }

        for n in arr.iter() {
            if let Some(v1) = map.get_mut(n) {
                if *v1 <= 0 {
                    continue;
                }
                *v1 -= 1;
            }

            if let Some(v2) = map.get_mut(&(n * 2)) {
                if *v2 <= 0 {
                    *map.entry(*n).or_insert(0) += 1;
                    continue;
                }
                *v2 -= 1;
            } else {
                *map.entry(*n).or_insert(0) += 1;
            }
        }
        map.iter().filter(|&(_, v)| *v != 0).count() == 0
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::can_reorder_doubled(vec![3, 1, 3, 6]));
    println!("{}", Solution::can_reorder_doubled(vec![4, -2, 2, -4]));
    println!("{}", Solution::can_reorder_doubled(vec![0, 0]));
    println!(
        "{}",
        Solution::can_reorder_doubled(vec![1, 2, 1, -8, 8, -4, 4, -4, 2, -2])
    );
}

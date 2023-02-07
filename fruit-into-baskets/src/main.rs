use std::collections::HashSet;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..fruits.len() {
            if res > fruits.len() / 2 {
                break;
            }
            let mut set = HashSet::new();
            set.insert(fruits[i]);
            let mut count = 1;
            for j in i + 1..fruits.len() {
                if set.contains(&fruits[j]) {
                    count += 1;
                } else if set.len() < 2 {
                    set.insert(fruits[j]);
                    count += 1;
                } else {
                    break;
                }
            }
            res = res.max(count);
        }
        res as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::total_fruit(vec![1, 2, 1]));
}

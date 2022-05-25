use std::collections::BTreeSet;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut set = BTreeSet::new();
        for i in 1..=n {
            set.insert(i.to_string());
        }
        set.iter().map(|x| x.parse::<i32>().unwrap()).collect()
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::lexical_order(13));
}

use std::collections::BTreeSet;
use std::collections::VecDeque;

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut set: BTreeSet<String> = BTreeSet::new();
        if k as usize > s.len() / 2 {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            return chars
                .iter()
                .fold(String::new(), |acc, cur| acc + &cur.to_string());
        }
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(s);
        while !queue.is_empty() {
            let s = queue.pop_front().unwrap();
            for i in 0..k as usize {
                let t = s.get(0..i).unwrap().to_string()
                    + s.get(i + 1..).unwrap()
                    + s.get(i..i + 1).unwrap();
                if !set.contains(&t) {
                    set.insert(t.clone());
                    queue.push_back(t);
                }
            }
        }
        set.into_iter().next().unwrap()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::orderly_queue("baaca".to_string(), 3));
    println!("{}", Solution::orderly_queue("adguvpsubc".to_string(), 2));
}

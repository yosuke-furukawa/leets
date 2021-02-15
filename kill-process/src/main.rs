use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn kill_process(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, key) in ppid.iter().enumerate() {
            map.entry(key).or_insert_with(Vec::new).push(pid[i]);
        }

        if !map.contains_key(&kill) {
            return vec![kill];
        }

        let mut set = HashSet::new();
        set.insert(kill);
        let mut queue = VecDeque::new();
        for id in map.get(&kill).unwrap() {
            queue.push_back(*id);
        }
        while !queue.is_empty() {
            let value = queue.pop_front().unwrap();
            set.insert(value);
            match map.get(&value) {
                Some(ids) => {
                    for id in ids {
                        queue.push_back(*id);
                    }
                }
                None => {
                    continue;
                }
            }
        }
        set.into_iter().collect::<Vec<i32>>()
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::kill_process(vec![1, 3, 10, 5], vec![3, 0, 5, 3], 5)
    );
    println!(
        "{:?}",
        Solution::kill_process(vec![1, 2, 3, 4], vec![4, 4, 4, 9], 4)
    );
    println!("{:?}", Solution::kill_process(vec![1], vec![0], 1));
}

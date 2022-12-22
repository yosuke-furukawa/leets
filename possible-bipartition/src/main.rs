use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        if dislikes.is_empty() {
            return true;
        }
        let mut map = HashMap::new();
        for d in dislikes.iter() {
            map.entry(d[0]).or_insert_with(HashSet::new).insert(d[1]);
            map.entry(d[1]).or_insert_with(HashSet::new).insert(d[0]);
        }
        let mut group1 = HashSet::new();
        let mut group2 = HashSet::new();
        let mut queue = VecDeque::from(map.keys().cloned().collect::<Vec<i32>>());
        let mut visited = HashSet::new();
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            if group1.contains(&node) {
                if let Some(neighbors) = map.get(&node) {
                    for &n in neighbors {
                        if group1.contains(&n) {
                            return false;
                        }
                        group2.insert(n);
                        queue.push_front(n);
                    }
                }
            } else if group2.contains(&node) {
                if let Some(neighbors) = map.get(&node) {
                    for &n in neighbors {
                        if group2.contains(&n) {
                            return false;
                        }
                        group1.insert(n);
                        queue.push_front(n);
                    }
                }
            } else {
                group1.insert(node);
                if let Some(neighbors) = map.get(&node) {
                    for &n in neighbors {
                        if group1.contains(&n) {
                            return false;
                        }
                        group2.insert(n);
                        queue.push_front(n);
                    }
                }
            }
        }
        true
    }
}

struct Solution;

#[macro_export]
macro_rules! grid {
    ( $([$( $x:expr ),*]),* ) => {
        {
            vec![
                $(
                    vec![$($x), *],
                )*
            ]
        }
    };
}

fn main() {
    println!(
        "{}",
        Solution::possible_bipartition(4, grid![[1, 2], [1, 3], [2, 4]])
    );
    println!(
        "{}",
        Solution::possible_bipartition(10, grid![[1, 2], [3, 4], [5, 6], [6, 7], [8, 9], [7, 8]])
    );
}

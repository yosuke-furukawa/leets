use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut map = HashMap::new();
        for e in edges {
            map.entry(e[0]).or_insert_with(HashSet::new).insert(e[1]);
            map.entry(e[1]).or_insert_with(HashSet::new).insert(e[0]);
        }
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(source);

        while !q.is_empty() {
            let node = q.pop_front().unwrap();
            if node == destination {
                return true;
            }
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            if let Some(neighbors) = map.get(&node) {
                for &n in neighbors {
                    if visited.contains(&n) {
                        continue;
                    }
                    q.push_back(n);
                }
            }
        }
        false
    }
}

struct Solution;

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
        Solution::valid_path(3, grid![[0, 1], [1, 2], [2, 0]], 0, 2)
    );
}

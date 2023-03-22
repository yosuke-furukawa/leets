use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut graph = HashMap::new();
        for road in roads {
            let (a, b, c) = (road[0], road[1], road[2]);
            graph.entry(a).or_insert(Vec::new()).push((b, c));
            graph.entry(b).or_insert(Vec::new()).push((a, c));
        }
        let mut queue = VecDeque::new();
        queue.push_back((1, std::i32::MAX - 1));
        let mut visited = vec![std::i32::MAX; n as usize + 1];
        while let Some((node, cost)) = queue.pop_front() {
            if visited[node as usize] <= cost {
                continue;
            }
            visited[node as usize] = cost;
            if let Some(neighbors) = graph.get(&node) {
                for (neighbor, neighbor_cost) in neighbors {
                    queue.push_back((*neighbor, (*neighbor_cost).min(cost)));
                }
            }
        }
        visited[n as usize]
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
        Solution::min_score(4, grid![[1, 2, 9], [2, 3, 6], [2, 4, 5], [1, 4, 7]])
    );
    println!(
        "{}",
        Solution::min_score(4, grid![[1, 2, 2], [1, 3, 4], [3, 4, 7]])
    );
}

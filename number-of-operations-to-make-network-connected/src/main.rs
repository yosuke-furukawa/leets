use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        if connections.len() < n as usize - 1 {
            return -1;
        }
        let mut graph = HashMap::new();
        for connection in connections {
            let (a, b) = (connection[0], connection[1]);
            graph.entry(a).or_insert(HashSet::new()).insert(b);
            graph.entry(b).or_insert(HashSet::new()).insert(a);
        }
        let mut queue = VecDeque::new();
        let mut visited = vec![false; n as usize];
        let mut ans = 0;
        for i in 0..n {
            if visited[i as usize] {
                continue;
            }
            queue.push_back(i);
            while let Some(node) = queue.pop_front() {
                if visited[node as usize] {
                    continue;
                }
                visited[node as usize] = true;
                if let Some(neighbors) = graph.get(&node) {
                    for neighbor in neighbors {
                        queue.push_back(*neighbor);
                    }
                }
            }
            ans += 1;
        }
        ans - 1
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
        Solution::make_connected(4, grid![[0, 1], [0, 2], [1, 2]])
    );
    println!(
        "{}",
        Solution::make_connected(6, grid![[0, 1], [0, 2], [0, 3], [1, 2], [1, 3]])
    );
    println!(
        "{}",
        Solution::make_connected(6, grid![[0, 1], [0, 2], [0, 3], [1, 2]])
    );
    println!(
        "{}",
        Solution::make_connected(
            12,
            grid![
                [1, 5],
                [1, 7],
                [1, 2],
                [1, 4],
                [3, 7],
                [4, 7],
                [3, 5],
                [0, 6],
                [0, 1],
                [0, 4],
                [2, 6],
                [0, 3],
                [0, 2]
            ]
        )
    );
}

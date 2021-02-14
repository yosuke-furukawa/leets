use std::collections::VecDeque;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut colors = vec![0; graph.len()];

        for i in 0..graph.len() {
            if colors[i] != 0 {
                continue;
            }
            let mut queue = VecDeque::<i32>::new();
            queue.push_back(i as i32);
            colors[i] = 1;

            while let Some(x) = queue.pop_front() {
                for &next in &graph[x as usize] {
                    if colors[next as usize] == 0 {
                        colors[next as usize] = -colors[x as usize];
                        queue.push_back(next);
                    } else if colors[next as usize] != -colors[x as usize] {
                        return false;
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
        Solution::is_bipartite(grid![[1, 3], [0, 2], [1, 3], [0, 2]])
    );
    println!(
        "{}",
        Solution::is_bipartite(grid![[1], [0], [4], [4], [2, 3]])
    );
}

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        if graph.len() == 1 {
            return 0;
        }
        let n = graph.len();
        let ending_mask = (1 << n) - 1;
        let mut seen = vec![vec![false; ending_mask]; n];
        let mut queue = vec![];

        for i in 0..n {
            queue.push((i, 1 << i));
            seen[i][1 << i] = true;
        }

        let mut steps = 0;
        while !queue.is_empty() {
            let mut next_queue = vec![];
            for (node, mask) in queue {
                for &neighbor in graph[node].iter() {
                    let next_mask = mask | (1 << neighbor);
                    if next_mask == ending_mask {
                        return steps + 1;
                    }
                    if !seen[neighbor as usize][next_mask] {
                        next_queue.push((neighbor as usize, next_mask));
                        seen[neighbor as usize][next_mask] = true;
                    }
                }
            }
            steps += 1;
            queue = next_queue;
        }

        -1
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
    println!("{}", Solution::shortest_path_length(grid![[1, 2, 3], [0], [0], [0]]));
}

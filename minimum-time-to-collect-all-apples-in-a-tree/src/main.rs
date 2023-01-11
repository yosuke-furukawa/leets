impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for e in edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            adj[a].push(b);
            adj[b].push(a);
        }
        fn dfs(current: usize, parent: usize, has_apple: &[bool], adj: &[Vec<usize>]) -> i32 {
            adj[current]
                .iter()
                .filter(|&&child| parent != child)
                .fold(0, |acc, &child| {
                    let sum_child = dfs(child, current, has_apple, adj);
                    acc + sum_child
                        + if sum_child > 0 || has_apple[child] {
                            2
                        } else {
                            0
                        }
                })
        }
        dfs(0, 0, &has_apple, &adj)
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
        Solution::min_time(
            7,
            grid![[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
            vec![false, false, true, false, true, true, false]
        )
    );
}

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut graph = vec![vec![HashSet::<i32>::new(); n as usize]; 2];
        for red in &red_edges {
            graph[0][red[0] as usize].insert(red[1]);
        }
        for blue in &blue_edges {
            graph[1][blue[0] as usize].insert(blue[1]);
        }
        let mut ans = vec![vec![n + n; n as usize]; 2];
        ans[0][0] = 0;
        ans[1][0] = 0;
        let mut q = VecDeque::<(i32, i32)>::new();
        q.push_back((0, 0));
        q.push_back((0, 1));
        while let Some((vert, color)) = q.pop_front() {
            for &next in &graph[(1 - color) as usize][vert as usize] {
                if ans[(1 - color) as usize][next as usize] == n + n {
                    ans[(1 - color) as usize][next as usize] =
                        ans[color as usize][vert as usize] + 1;
                    q.push_back((next, 1 - color));
                }
            }
        }
        let mut res = vec![0; n as usize];
        for i in 0..n as usize {
            let t = std::cmp::min(ans[0][i], ans[1][i]);
            res[i] = if t == n + n { -1 } else { t }
        }
        res
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
        "{:?}",
        Solution::shortest_alternating_paths(3, grid![[0, 1], [1, 2]], grid![])
    );
}

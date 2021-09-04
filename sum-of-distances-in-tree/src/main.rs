use std::collections::HashSet;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![0; n as usize];
        let mut count = vec![1; n as usize];
        let mut graph: Vec<HashSet<i32>> = vec![HashSet::new(); n as usize];
        for edge in edges {
            graph.get_mut(edge[0] as usize).unwrap().insert(edge[1]);
            graph.get_mut(edge[1] as usize).unwrap().insert(edge[0]);
        }
        fn dfs(node: i32, parent: i32, graph: &[HashSet<i32>], count: &mut [i32], ans: &mut [i32]) {
            for child in graph.get(node as usize).unwrap() {
                if *child != parent {
                    dfs(*child, node, graph, count, ans);
                    count[node as usize] += count[*child as usize];
                    ans[node as usize] += ans[*child as usize] + count[*child as usize];
                }
            }
        }
        fn dfs2(
            node: i32,
            parent: i32,
            graph: &[HashSet<i32>],
            count: &[i32],
            ans: &mut [i32],
            n: i32,
        ) {
            for child in graph.get(node as usize).unwrap() {
                if *child != parent {
                    ans[*child as usize] =
                        ans[node as usize] - count[*child as usize] + n - count[*child as usize];
                    dfs2(*child, node, graph, count, ans, n);
                }
            }
        }
        dfs(0, -1, &graph, &mut count, &mut ans);
        dfs2(0, -1, &graph, &count, &mut ans, n);
        ans
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
        Solution::sum_of_distances_in_tree(6, grid![[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]])
    );
}

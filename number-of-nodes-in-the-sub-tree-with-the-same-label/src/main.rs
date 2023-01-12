use std::collections::HashMap;

impl Solution {
    fn dfs(
        node: usize,
        graph: &Vec<Vec<usize>>,
        labels: &[char],
        visited: &mut Vec<bool>,
        counts: &mut Vec<HashMap<char, i32>>,
        res: &mut Vec<i32>,
    ) {
        visited[node] = true;
        let mut count = HashMap::new();
        count.insert(labels[node], 1);
        for &next in &graph[node] {
            if visited[next] {
                continue;
            }
            Self::dfs(next, graph, labels, visited, counts, res);
            for (c, n) in &counts[next] {
                *count.entry(*c).or_insert(0) += n;
            }
        }
        counts[node] = count;
        res[node] = counts[node][&labels[node]];
    }
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        let labels: Vec<char> = labels.chars().collect();
        let mut visited = vec![false; n];
        let mut counts = vec![HashMap::new(); n];
        let mut res = vec![0; n];
        Self::dfs(0, &graph, &labels, &mut visited, &mut counts, &mut res);
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
        Solution::count_sub_trees(
            7,
            grid![[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
            "abaedcd".to_string()
        )
    );
}

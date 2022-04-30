use std::collections::BTreeSet;
use std::collections::HashMap;

type Edge = (usize, f64);

impl Solution {
    fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let m = equations.len();
        let mut symbols: BTreeSet<String> = BTreeSet::new();
        let mut ids: HashMap<String, usize> = HashMap::new();
        for eq in &equations {
            symbols.insert(eq[0].clone());
            symbols.insert(eq[1].clone());
        }
        for (i, s) in symbols.into_iter().enumerate() {
            ids.insert(s, i);
        }
        let n = ids.len();
        let mut graph: Vec<Vec<Edge>> = vec![vec![]; n];

        for i in 0..m {
            let u = ids[&equations[i][0]];
            let v = ids[&equations[i][1]];
            graph[u].push((v, values[i]));
            graph[v].push((u, 1.0 / values[i]));
        }
        let mut res = vec![];
        for query in queries {
            if ids.contains_key(&query[0]) && ids.contains_key(&query[1]) {
                let u = ids[&query[0]];
                let v = ids[&query[1]];
                let mut product = -1.0;
                let mut visited = vec![false; n];
                let mut path: Vec<f64> = vec![];
                Self::dfs(u, v, &mut visited, &mut path, &mut product, &graph);
                res.push(product);
            } else {
                res.push(-1.0);
            }
        }
        res
    }

    fn dfs(
        u: usize,
        v: usize,
        visited: &mut Vec<bool>,
        path: &mut Vec<f64>,
        product: &mut f64,
        graph: &[Vec<Edge>],
    ) {
        visited[u] = true;
        if u == v {
            *product = path.iter().fold(1.0, |a, v| a * v);
        } else {
            for e in &graph[u] {
                if !visited[e.0] {
                    path.push(e.1);
                    Self::dfs(e.0, v, visited, path, product, graph);
                    path.pop();
                }
            }
        }
        visited[u] = false;
    }
}

struct Solution;

fn main() {
    let equations = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "c".to_string()],
    ];
    let values = vec![2.0, 3.0];
    let queries = vec![
        vec!["a".to_string(), "c".to_string()],
        vec!["b".to_string(), "a".to_string()],
        vec!["a".to_string(), "e".to_string()],
        vec!["a".to_string(), "a".to_string()],
        vec!["x".to_string(), "x".to_string()],
    ];
    println!("{:?}", Solution::calc_equation(equations, values, queries));
}

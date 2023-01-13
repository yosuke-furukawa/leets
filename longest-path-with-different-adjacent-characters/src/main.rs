impl Solution {
    fn dfs(u: usize, graph: &Vec<Vec<usize>>, s: &[u8], res: &mut Vec<i32>) -> i32 {
        let (mut l1, mut l2) = (0, 0);

        for &v in graph[u].iter() {
            let l = Self::dfs(v, graph, s, res);
            if s[u] != s[v] {
                if l > l1 {
                    l2 = l1;
                    l1 = l
                } else if l > l2 {
                    l2 = l
                }
            }
        }

        res[0] = res[0].max(l1 + l2 + 1);
        l1 + 1
    }
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let mut res = vec![0];
        let mut graph = vec![vec![]; s.len()];

        for (v, &u) in parent.iter().enumerate() {
            if u != -1 {
                graph[u as usize].push(v)
            }
        }

        Self::dfs(0, &graph, s.as_bytes(), &mut res);
        res[0]
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::longest_path(vec![-1, 0, 0, 1, 1, 2], "abacbe".to_string())
    );
}

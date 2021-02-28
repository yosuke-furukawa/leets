use std::collections::VecDeque;

impl Solution {
    pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        let mut degree = vec![0; n];
        for edge in relations {
            let x = edge[0] as usize - 1;
            let y = edge[1] as usize - 1;
            adj[x].push(y);
            degree[y] += 1;
        }
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();

        for i in 0..n {
            if degree[i] == 0 {
                visited[i] = true;
                queue.push_back(i);
            }
        }

        let mut res = 0;
        while !queue.is_empty() {
            let n = queue.len();
            res += 1;
            for _ in 0..n {
                let u = queue.pop_front().unwrap();
                for &v in adj[u].iter() {
                    degree[v] -= 1;
                    if !visited[v] && degree[v] == 0 {
                        visited[v] = true;
                        queue.push_back(v);
                    }
                }
            }
        }

        if visited.into_iter().all(|x| x) {
            res
        } else {
            -1
        }
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::minimum_semesters(3, vec![vec![1, 3], vec![2, 3]])
    );
}

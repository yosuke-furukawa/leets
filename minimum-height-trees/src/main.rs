use std::collections::VecDeque;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n < 2 {
            return vec![0];
        }
        let mut node_exist = vec![true; n as usize];
        let mut in_degrees = vec![0; n as usize];
        let mut edges_relate = vec![vec![]; n as usize];
        for e in edges {
            in_degrees[e[0] as usize] += 1;
            in_degrees[e[1] as usize] += 1;
            edges_relate[e[0] as usize].push(e[1] as usize);
            edges_relate[e[1] as usize].push(e[0] as usize);
        }
        let mut q = VecDeque::new();
        let mut remain = n as usize;
        for (i, v) in in_degrees.iter().enumerate().take(n as usize) {
            if *v == 1 {
                q.push_back(i)
            }
        }
        while remain > q.len() {
            for _ in 0..q.len() {
                let leaf = q.pop_front().unwrap();
                node_exist[leaf] = false;
                remain -= 1;
                for &i in edges_relate[leaf].iter() {
                    if node_exist[i] {
                        in_degrees[i] -= 1;
                        if in_degrees[i] == 1 {
                            q.push_back(i)
                        }
                    }
                }
            }
        }
        q.into_iter().map(|x| x as i32).collect()
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
        Solution::find_min_height_trees(4, grid![[1, 0], [1, 2], [1, 3]])
    );
}

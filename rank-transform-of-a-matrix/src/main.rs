use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (row, col) = (matrix.len(), matrix[0].len());
        let mut union_find = UnionFind::new(row * col);

        for i in 0..row {
            let mut map = HashMap::new();
            for j in 0..col {
                map.entry(matrix[i][j])
                    .or_insert(Vec::new())
                    .push(i * col + j);
            }
            for (_key, val) in map {
                for k in 0..(val.len() - 1) {
                    union_find.union(val[k], val[k + 1]);
                }
            }
        }
        for j in 0..col {
            let mut map = HashMap::new();
            for i in 0..row {
                map.entry(matrix[i][j])
                    .or_insert(Vec::new())
                    .push(i * col + j);
            }
            for (_key, val) in map {
                for k in 0..(val.len() - 1) {
                    union_find.union(val[k], val[k + 1]);
                }
            }
        }
        let mut adj_list = vec![Vec::new(); row * col];
        let mut in_degree = vec![0; row * col];

        for i in 0..row {
            let mut vec_pair = vec![(0, 0); col];
            for j in 0..col {
                vec_pair[j] = (matrix[i][j], j);
            }
            vec_pair.sort();
            for j in 0..(col - 1) {
                if vec_pair[j].0 != vec_pair[j + 1].0 {
                    let u_root = union_find.root(i * col + vec_pair[j].1);
                    let v_root = union_find.root(i * col + vec_pair[j + 1].1);
                    adj_list[u_root].push(v_root);
                    in_degree[v_root] += 1;
                }
            }
        }
        for j in 0..col {
            let mut vec_pair = vec![(0, 0); row];
            for i in 0..row {
                vec_pair[i] = (matrix[i][j], i);
            }
            vec_pair.sort();
            for i in 0..(row - 1) {
                if vec_pair[i].0 != vec_pair[i + 1].0 {
                    let u_root = union_find.root(vec_pair[i].1 * col + j);
                    let v_root = union_find.root(vec_pair[i + 1].1 * col + j);
                    adj_list[u_root].push(v_root);
                    in_degree[v_root] += 1;
                }
            }
        }
        let mut ans = vec![1; row * col];
        let mut queue = VecDeque::new();
        for i in 0..row * col {
            if union_find.root(i) == i && in_degree[i] == 0 {
                queue.push_back(i);
            }
        }
        while let Some(node) = queue.pop_back() {
            for &v in &adj_list[node] {
                ans[v] = ans[v].max(ans[node] + 1);
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }
        let mut result = vec![vec![0; col]; row];
        for i in 0..row {
            for j in 0..col {
                result[i][j] = ans[union_find.root(i * col + j)];
            }
        }
        result
    }
}

struct Solution;

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            ancestor: (0..n as u32).collect(),
            size: vec![1; n],
        }
    }
    pub fn union(&mut self, index_1: usize, index_2: usize) {
        let (root_1, root_2) = (self.root(index_1), self.root(index_2));
        if root_1 == root_2 {
            return;
        }
        if self.size[root_1] > self.size[root_2] {
            self.ancestor[root_2] = root_1 as u32;
            self.size[root_1] += self.size[root_2];
        } else {
            self.ancestor[root_1] = root_2 as u32;
            self.size[root_2] += self.size[root_1];
        }
    }
    pub fn root(&mut self, mut index: usize) -> usize {
        while index != self.ancestor[index] as usize {
            self.ancestor[index] = self.ancestor[self.ancestor[index] as usize];
            index = self.ancestor[index] as usize;
        }
        index
    }
}

struct UnionFind {
    ancestor: Vec<u32>,
    size: Vec<u32>,
}

fn main() {
    println!(
        "{:?}",
        Solution::matrix_rank_transform(vec![vec![1, 2], vec![3, 4]])
    );
}

use std::collections::HashSet;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(20000);
        for stone in stones.iter() {
            uf.union(stone[0] as usize, stone[1] as usize + 10000);
        }
        let mut seen = HashSet::new();
        for stone in stones.iter() {
            seen.insert(uf.find(stone[0] as usize));
        }
        (stones.len() - seen.len()) as i32
    }
}

#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for (i, p) in parent.iter_mut().enumerate().take(n) {
            *p = i;
        }
        UnionFind { parent }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let i = self.find(x);
        self.parent[i] = self.find(y);
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
        Solution::remove_stones(grid![[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]])
    );
}

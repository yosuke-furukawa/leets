impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let index_of = |row: usize, col: usize| -> usize { row * cols + col };
        let mut edges = Vec::new();
        for row in 0..rows {
            for col in 0..cols {
                let idx = index_of(row, col);
                if row > 0 {
                    edges.push((
                        grid[row - 1][col].max(grid[row][col]),
                        index_of(row - 1, col),
                        idx,
                    ));
                }
                if col > 0 {
                    edges.push((
                        grid[row][col - 1].max(grid[row][col]),
                        index_of(row, col - 1),
                        idx,
                    ));
                }
            }
        }

        edges.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        let mut uf = UnionFind::new(rows * cols);
        for edge in edges {
            uf.union(edge.1, edge.2);
            if uf.check_connected(0, rows * cols - 1) {
                return edge.0;
            }
        }
        0
    }
}

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    sz: Vec<usize>,
    pub set_count: usize,
}

#[allow(unused)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for (i, item) in parent.iter_mut().enumerate().take(n) {
            *item = i;
        }
        UnionFind {
            parent,
            rank: vec![0; n],
            sz: vec![1; n],
            set_count: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x;
                self.sz[root_x] += self.sz[root_y];
            }
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y;
                self.sz[root_y] += self.sz[root_x];
            }
            _ => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
                self.sz[root_x] += self.sz[root_y];
            }
        }
        self.set_count -= 1;
        true
    }

    pub fn check_connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
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
    println!("{}", Solution::swim_in_water(grid![[0, 2], [1, 3]]));
}

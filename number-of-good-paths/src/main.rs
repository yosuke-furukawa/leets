use std::cmp::max;

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = vals.len();
        let mut res = 0;
        let mut edges = edges;

        edges.sort_by(|x, y| {
            max(vals[x[0] as usize], vals[x[1] as usize])
                .cmp(&max(vals[y[0] as usize], vals[y[1] as usize]))
        });

        let mut uf = UnionFind::new(n, vals);

        for edge in edges {
            let (x, y) = (edge[0] as usize, edge[1] as usize);
            res += uf.union(x, y);
        }

        res + n as i32
    }
}

struct UnionFind {
    root: Vec<usize>,
    size: Vec<i32>,
    vals: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize, vals: Vec<i32>) -> Self {
        let mut root: Vec<usize> = vec![0; n];
        let size: Vec<i32> = vec![1; n];
        for i in 0..n {
            root[i] = i;
        }

        UnionFind { root, size, vals }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.root[x] != x {
            self.root[x] = Self::find(self, self.root[x]);
        }

        self.root[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> i32 {
        let mut count = 0;
        let root_x = Self::find(self, x);
        let root_y = Self::find(self, y);

        if self.vals[root_x] == self.vals[root_y] {
            count += self.size[root_x] * self.size[root_y];

            self.root[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else if self.vals[root_x] > self.vals[root_y] {
            self.root[root_y] = root_x;
        } else {
            self.root[root_x] = root_y;
        }

        count
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
        Solution::number_of_good_paths(vec![1, 3, 2, 1, 3], grid![[0, 1], [0, 2], [2, 3], [2, 4]])
    );
}

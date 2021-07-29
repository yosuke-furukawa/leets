use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mat = mat;
        let mut queue = VecDeque::new();
        for (x, m) in mat.iter().enumerate() {
            for (y, n) in m.iter().enumerate() {
                if *n == 1 {
                    queue.push_back((x, y));
                }
            }
        }

        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            let prev = mat[x][y];
            let mut min = 1_000_000_000;
            if x > 0 {
                min = min.min(mat[x - 1][y]);
            }
            if y > 0 {
                min = min.min(mat[x][y - 1]);
            }
            if x < mat.len() - 1 {
                min = min.min(mat[x + 1][y]);
            }
            if y < mat[0].len() - 1 {
                min = min.min(mat[x][y + 1]);
            }
            if min == 0 || min + 1 == prev {
                continue;
            }
            mat[x][y] = min + 1;
            queue.push_back((x, y));
        }

        mat
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
        Solution::update_matrix(grid![[0, 0, 0], [0, 1, 0], [0, 0, 0]])
    );
    println!(
        "{:?}",
        Solution::update_matrix(grid![[0, 0, 0], [0, 1, 0], [1, 1, 1]])
    );
    println!("{:?}", Solution::update_matrix(grid![[0], [1]]));
}

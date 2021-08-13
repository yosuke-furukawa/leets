impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut zeros = vec![];
        for (i, matrice) in matrix.iter().enumerate() {
            for (j, m) in matrice.iter().enumerate() {
                if m == &0 {
                    zeros.push((i, j));
                }
            }
        }

        while !zeros.is_empty() {
            let (col, row) = zeros.pop().unwrap();
            for c in 0..col {
                matrix[c][row] = 0;
            }
            for c in col + 1..matrix.len() {
                matrix[c][row] = 0;
            }
            for r in 0..row {
                matrix[col][r] = 0;
            }
            for r in row + 1..matrix[0].len() {
                matrix[col][r] = 0;
            }
        }
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
    let mut grid = grid![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
    Solution::set_zeroes(&mut grid);
    println!("{:?}", grid);
}

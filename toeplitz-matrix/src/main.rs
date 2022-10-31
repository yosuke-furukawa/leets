impl Solution {
    fn is_toeplitz(matrix: &[Vec<i32>], sx: usize, sy: usize) -> bool {
        let mut x = sx;
        let mut y = sy;
        let n = matrix[y][x];
        while y < matrix.len() && x < matrix[0].len() {
            if n != matrix[y][x] {
                return false;
            }
            y += 1;
            x += 1;
        }
        true
    }
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for sx in 0..matrix[0].len() {
            if !Self::is_toeplitz(&matrix, sx, 0) {
                return false;
            }
        }
        for sy in 0..matrix.len() {
            if !Self::is_toeplitz(&matrix, 0, sy) {
                return false;
            }
        }
        true
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
        Solution::is_toeplitz_matrix(grid![[1, 2, 3, 4], [5, 1, 2, 3], [9, 5, 1, 2]])
    );
    println!("{}", Solution::is_toeplitz_matrix(grid![[1, 2], [2, 2]]));
}

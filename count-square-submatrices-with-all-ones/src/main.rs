impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if i > 0 && j > 0 && matrix[i][j] > 0 {
                    matrix[i][j] = 1 + matrix[i - 1][j]
                        .min(matrix[i - 1][j - 1])
                        .min(matrix[i][j - 1]);
                }
                count += matrix[i][j];
            }
        }
        count
    }
}

struct Solution;

#[macro_export]
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
        Solution::count_squares(grid![[0, 1, 1, 1], [1, 1, 1, 1], [0, 1, 1, 1]])
    );
}

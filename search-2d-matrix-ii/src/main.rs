impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut x = 0;
        let mut y = matrix.len() - 1;
        while y < matrix.len() && x < matrix[y].len() {
            match (matrix[y][x], target) {
                (v, t) if v < t => x += 1,
                (v, t) if v > t => y -= 1,
                _ => {
                    return true;
                }
            }
        }
        false
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
        Solution::search_matrix(
            grid![
                [1, 4, 7, 11, 15],
                [2, 5, 8, 12, 19],
                [3, 6, 9, 16, 22],
                [10, 13, 14, 17, 24],
                [18, 21, 23, 26, 30]
            ],
            5
        )
    );
    println!(
        "{}",
        Solution::search_matrix(
            grid![
                [1, 4, 7, 11, 15],
                [2, 5, 8, 12, 19],
                [3, 6, 9, 16, 22],
                [10, 13, 14, 17, 24],
                [18, 21, 23, 26, 30]
            ],
            20
        )
    );
}

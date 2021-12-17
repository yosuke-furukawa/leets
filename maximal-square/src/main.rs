impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let mut dp: Vec<Vec<i32>> = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        let mut max = 0;
        for i in 1..=matrix.len() {
            for j in 1..=matrix[0].len() {
                if matrix[i - 1][j - 1] == '1' {
                    dp[i][j] =
                        std::cmp::min(dp[i - 1][j - 1], std::cmp::min(dp[i - 1][j], dp[i][j - 1]))
                            + 1;
                    max = std::cmp::max(max, dp[i][j]);
                }
            }
        }
        max * max
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
        Solution::maximal_square(grid![
            ['1', '0', '1', '0', '0'],
            ['1', '0', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['1', '0', '0', '1', '0']
        ])
    );
}

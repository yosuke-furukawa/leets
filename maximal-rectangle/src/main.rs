impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let n = matrix.len();
        let m = matrix[0].len();
        let mut dp = vec![vec![0; m]; n];
        let mut max_area = 0;
        for r in 0..n {
            for c in 0..m {
                if matrix[r][c] == '0' {
                    continue;
                }
                dp[r][c] += if r > 0 { dp[r - 1][c] + 1 } else { 1 };
                let mut min = dp[r][c];
                for k in (0..=c).rev() {
                    if dp[r][k] == 0 {
                        break;
                    }
                    min = dp[r][k].min(min);
                    max_area = max_area.max(min * (c - k + 1));
                }
            }
        }
        max_area as i32
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
        Solution::maximal_rectangle(grid![
            ['1', '0', '1', '0', '0'],
            ['1', '0', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['1', '0', '0', '1', '0']
        ])
    );
}

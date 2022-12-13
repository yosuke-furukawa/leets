impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut dp = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                let mut min = std::i32::MAX;
                if i > 0 {
                    let l = 0.max(j as i32 - 1) as usize;
                    let r = (n - 1).min(j + 1);
                    for k in l..=r {
                        min = min.min(dp[i - 1][k]);
                    }
                } else {
                    min = 0;
                }
                dp[i][j] = matrix[i][j] + min;
            }
        }
        *dp[n - 1].iter().min().unwrap()
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
        Solution::min_falling_path_sum(grid![[2, 1, 3], [6, 5, 4], [7, 8, 9]])
    );
}

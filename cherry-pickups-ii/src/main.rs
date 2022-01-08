use std::collections::HashMap;

impl Solution {
    fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut memo: HashMap<(usize, usize, usize), i32> = HashMap::new();
        Self::dp(0, 0, m - 1, &mut memo, &grid, n, m)
    }
    fn dp(
        i: usize,
        j: usize,
        k: usize,
        memo: &mut HashMap<(usize, usize, usize), i32>,
        grid: &[Vec<i32>],
        n: usize,
        m: usize,
    ) -> i32 {
        if let Some(&res) = memo.get(&(i, j, k)) {
            return res;
        }

        let mut res = if k != j {
            grid[i][j] + grid[i][k]
        } else {
            grid[i][j]
        };

        if i + 1 < n {
            let mut max = 0;
            if j > 0 && k > 0 {
                max = max.max(Self::dp(i + 1, j - 1, k - 1, memo, grid, n, m));
            }
            if k > 0 {
                max = max.max(Self::dp(i + 1, j, k - 1, memo, grid, n, m));
            }
            if j + 1 < m && k > 0 {
                max = max.max(Self::dp(i + 1, j + 1, k - 1, memo, grid, n, m));
            }
            if j > 0 {
                max = max.max(Self::dp(i + 1, j - 1, k, memo, grid, n, m));
            }
            max = max.max(Self::dp(i + 1, j, k, memo, grid, n, m));
            if j + 1 < m {
                max = max.max(Self::dp(i + 1, j + 1, k, memo, grid, n, m));
            }
            if j > 0 && k + 1 < m {
                max = max.max(Self::dp(i + 1, j - 1, k + 1, memo, grid, n, m));
            }
            if k + 1 < m {
                max = max.max(Self::dp(i + 1, j, k + 1, memo, grid, n, m));
            }
            if j + 1 < m && k + 1 < m {
                max = max.max(Self::dp(i + 1, j + 1, k + 1, memo, grid, n, m));
            }
            res += max;
        }
        memo.insert((i, j, k), res);
        res
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
        Solution::cherry_pickup(grid![[3, 1, 1], [2, 5, 1], [1, 5, 5], [2, 1, 1]])
    );
}

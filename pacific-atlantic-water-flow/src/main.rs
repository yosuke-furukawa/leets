use std::collections::HashSet;

impl Solution {
    fn dfs(
        matrix: &[Vec<i32>],
        row: i32,
        col: i32,
        reachable: &mut HashSet<(i32, i32)>,
        max_row: i32,
        max_col: i32,
    ) {
        reachable.insert((row, col));
        let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        for direction in directions.iter() {
            let nrow = row + direction.0;
            let ncol = col + direction.1;
            if nrow < 0 || nrow > max_row || ncol < 0 || ncol > max_col {
                continue;
            }
            if reachable.contains(&(nrow, ncol)) {
                continue;
            }
            if matrix[nrow as usize][ncol as usize] < matrix[row as usize][col as usize] {
                continue;
            }
            Self::dfs(matrix, nrow, ncol, reachable, max_row, max_col);
        }
    }
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut results = vec![];
        if matrix.is_empty() || matrix[0].is_empty() {
            return results;
        }
        let mut pacific_reachable: HashSet<(i32, i32)> = HashSet::new();
        let mut atlantic_reachable: HashSet<(i32, i32)> = HashSet::new();

        let max_row = (matrix.len() - 1) as i32;
        let max_col = (matrix[0].len() - 1) as i32;

        for i in 0..=max_row {
            Self::dfs(&matrix, i, 0, &mut pacific_reachable, max_row, max_col);
            Self::dfs(
                &matrix,
                i,
                max_col,
                &mut atlantic_reachable,
                max_row,
                max_col,
            );
        }

        for i in 0..=max_col {
            Self::dfs(&matrix, 0, i, &mut pacific_reachable, max_row, max_col);
            Self::dfs(
                &matrix,
                max_row,
                i,
                &mut atlantic_reachable,
                max_row,
                max_col,
            );
        }

        for r in 0..=max_row {
            for c in 0..=max_col {
                if pacific_reachable.contains(&(r, c)) && atlantic_reachable.contains(&(r, c)) {
                    results.push(vec![r, c]);
                }
            }
        }
        results
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
        Solution::pacific_atlantic(grid![
            [1, 2, 2, 3, 5],
            [3, 2, 3, 4, 4],
            [2, 4, 5, 3, 1],
            [6, 7, 1, 4, 5],
            [5, 1, 1, 2, 4]
        ])
    );
    println!(
        "{:?}",
        Solution::pacific_atlantic(grid![[1, 1], [1, 1], [1, 1]])
    );
}

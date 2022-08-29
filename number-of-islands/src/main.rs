use std::collections::HashSet;

impl Solution {
    fn dfs(grid: &[Vec<char>], i: i32, j: i32, visited: &mut HashSet<(usize, usize)>) {
        if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
            return;
        }
        if visited.contains(&(i as usize, j as usize)) {
            return;
        }
        if grid[i as usize][j as usize] == '1' {
            visited.insert((i as usize, j as usize));
            Self::dfs(grid, i - 1, j, visited);
            Self::dfs(grid, i + 1, j, visited);
            Self::dfs(grid, i, j + 1, visited);
            Self::dfs(grid, i, j - 1, visited);
        }
    }
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut visited = HashSet::new();
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();
        if n == 0 {
            return 0;
        }

        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' && !visited.contains(&(i, j)) {
                    res += 1;
                    Self::dfs(&grid, i as i32, j as i32, &mut visited);
                }
            }
        }
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
        Solution::num_islands(grid![
            ['1', '1', '1', '1', '0'],
            ['1', '1', '0', '1', '0'],
            ['1', '1', '0', '0', '0'],
            ['0', '0', '0', '0', '0']
        ])
    );

    println!(
        "{}",
        Solution::num_islands(grid![
            ['1', '1', '0', '0', '0'],
            ['1', '1', '0', '0', '0'],
            ['0', '0', '1', '0', '0'],
            ['0', '0', '0', '1', '1']
        ])
    );

    println!(
        "{}",
        Solution::num_islands(grid![['1', '1', '1'], ['0', '1', '0'], ['1', '1', '1']])
    );
}

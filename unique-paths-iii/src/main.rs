use std::collections::HashSet;

impl Solution {
    fn is_valid(position: (i32, i32), path: &HashSet<(i32, i32)>, grid: &Vec<Vec<i32>>) -> bool {
        if position.0 < 0 || position.1 < 0 {
            return false;
        }
        if position.0 >= grid.len() as i32 || position.1 >= grid[0].len() as i32 {
            return false;
        }
        if grid[position.0 as usize][position.1 as usize] == -1 {
            return false;
        }
        if path.contains(&position) {
            return false;
        }
        true
    }
    fn backtrack(
        position: (i32, i32),
        path: &mut HashSet<(i32, i32)>,
        grid: &Vec<Vec<i32>>,
        result: &mut i32,
        paths: usize,
    ) {
        println!(
            "{:?}, {:?}, {}",
            position, path, grid[position.0 as usize][position.1 as usize]
        );

        if grid[position.0 as usize][position.1 as usize] == 2 && path.len() == paths {
            *result += 1;
            return;
        }
        let n = (position.0 - 1, position.1);
        let e = (position.0, position.1 + 1);
        let w = (position.0, position.1 - 1);
        let s = (position.0 + 1, position.1);
        if Self::is_valid(n, path, grid) {
            path.insert(n);
            Self::backtrack(n, path, grid, result, paths);
            path.remove(&n);
        }
        if Self::is_valid(e, path, grid) {
            path.insert(e);
            Self::backtrack(e, path, grid, result, paths);
            path.remove(&e);
        }
        if Self::is_valid(w, path, grid) {
            path.insert(w);
            Self::backtrack(w, path, grid, result, paths);
            path.remove(&w);
        }
        if Self::is_valid(s, path, grid) {
            path.insert(s);
            Self::backtrack(s, path, grid, result, paths);
            path.remove(&s);
        }
    }
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut paths = 0;
        let mut start = vec![];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] != -1 {
                    paths += 1;
                }
                if grid[i][j] == 1 {
                    start = vec![i, j];
                }
            }
        }

        if start.is_empty() {
            return 0;
        }

        let mut result = 0;
        let start_pos = (start[0] as i32, start[1] as i32);
        let mut path = HashSet::new();
        path.insert(start_pos);
        Self::backtrack(start_pos, &mut path, &grid, &mut result, paths);

        result
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
        Solution::unique_paths_iii(grid![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, -1]])
    );
}

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut q = Vec::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, &x) in row.iter().enumerate() {
                if x == 1 {
                    q.push((i, j));
                }
            }
        }
        if q.is_empty() || q.len() == grid.len() * grid[0].len() {
            return -1;
        }
        let mut res = 0;
        while !q.is_empty() {
            let (i, j) = q.remove(0);
            for &(di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (i, j) = (i as i32 + di, j as i32 + dj);
                if i >= 0
                    && i < grid.len() as i32
                    && j >= 0
                    && j < grid[0].len() as i32
                    && grid[i as usize][j as usize] == 0
                {
                    grid[i as usize][j as usize] =
                        grid[i as usize - di as usize][j as usize - dj as usize] + 1;
                    res = res.max(grid[i as usize][j as usize] - 1);
                    q.push((i as usize, j as usize));
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
        Solution::max_distance(grid![[1, 0, 1], [0, 0, 0], [1, 0, 1]])
    );
}

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let grid = &mut { grid };
        let mut rotten_frontier = Vec::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 2 {
                    rotten_frontier.push((y, x));
                }
            }
        }

        let rows = grid.len();
        let cols = grid[0].len();
        let mut minutes = 0;
        loop {
            let mut new_rotten_frontier = Vec::new();
            for (y, x) in rotten_frontier {
                if y > 0 && grid[y - 1][x] == 1 {
                    grid[y - 1][x] = 2;
                    new_rotten_frontier.push((y - 1, x));
                }
                if y + 1 < rows && grid[y + 1][x] == 1 {
                    grid[y + 1][x] = 2;
                    new_rotten_frontier.push((y + 1, x));
                }
                if x > 0 && grid[y][x - 1] == 1 {
                    grid[y][x - 1] = 2;
                    new_rotten_frontier.push((y, x - 1));
                }
                if x + 1 < cols && grid[y][x + 1] == 1 {
                    grid[y][x + 1] = 2;
                    new_rotten_frontier.push((y, x + 1));
                }
            }
            if new_rotten_frontier.is_empty() {
                break;
            }
            minutes += 1;
            rotten_frontier = new_rotten_frontier;
        }

        for row in grid {
            for cell in row {
                if *cell == 1 {
                    return -1;
                }
            }
        }
        minutes
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
        Solution::oranges_rotting(grid![[2, 1, 1], [1, 1, 0], [0, 1, 1]])
    );
}

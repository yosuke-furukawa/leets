use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        let mut max = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if set.contains(&(i, j)) {
                    continue;
                }

                if grid[i][j] == 0 {
                    continue;
                }

                let mut queue = VecDeque::new();
                queue.push_back((i, j));
                let mut count = 0;
                while !queue.is_empty() {
                    if let Some(pos) = queue.pop_front() {
                        if set.contains(&pos) {
                            continue;
                        }

                        set.insert(pos);
                        count += 1;
                        if pos.0 > 0 && grid[pos.0 - 1][pos.1] == 1 {
                            queue.push_back((pos.0 - 1, pos.1));
                        }
                        if pos.1 < grid[0].len() - 1 && grid[pos.0][pos.1 + 1] == 1 {
                            queue.push_back((pos.0, pos.1 + 1));
                        }
                        if pos.1 > 0 && grid[pos.0][pos.1 - 1] == 1 {
                            queue.push_back((pos.0, pos.1 - 1));
                        }
                        if pos.0 < grid.len() - 1 && grid[pos.0 + 1][pos.1] == 1 {
                            queue.push_back((pos.0 + 1, pos.1));
                        }
                    }
                }
                max = max.max(count);
            }
        }
        max
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
        Solution::max_area_of_island(grid![
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ])
    );
}

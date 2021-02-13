use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let mut queue = VecDeque::new();
        let ylen = grid.len() as i32;
        let xlen = grid[0].len() as i32;
        let mut answer = vec![vec![-1; xlen as usize]; ylen as usize];

        let dx = [0, 1, 1, 1, 0, -1, -1, -1];
        let dy = [-1, -1, 0, 1, 1, 1, 0, -1];

        queue.push_back((0, 0));
        answer[0][0] = 1;
        while !queue.is_empty() {
            let location = queue.pop_front().unwrap();
            if location.0 == xlen - 1 && location.1 == ylen - 1 {
                break;
            }
            if grid[location.1 as usize][location.0 as usize] == 1 {
                break;
            }
            for i in 0..8 {
                let x = location.0 + dx[i];
                let y = location.1 + dy[i];
                let ans = answer[location.1 as usize][location.0 as usize];

                if y >= 0 && y < ylen && x >= 0 && x < xlen && grid[y as usize][x as usize] == 0 {
                    if answer[y as usize][x as usize] != -1 {
                        continue;
                    }
                    answer[y as usize][x as usize] = ans + 1;
                    queue.push_back((x, y));
                }
            }
        }
        answer[(ylen - 1) as usize][(xlen - 1) as usize]
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
        Solution::shortest_path_binary_matrix(grid![[0, 1], [1, 0]])
    );
    println!(
        "{}",
        Solution::shortest_path_binary_matrix(grid![[0, 0, 0], [1, 1, 0], [1, 1, 0]])
    );
    println!(
        "{}",
        Solution::shortest_path_binary_matrix(grid![[0, 0, 0], [1, 1, 0], [1, 1, 1]])
    );
}

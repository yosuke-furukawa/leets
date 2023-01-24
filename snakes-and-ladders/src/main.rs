use std::collections::VecDeque;

impl Solution {
    fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut moves: Vec<i32> = vec![0; n * n];
        let mut k = 0;
        for i in (0..n).rev() {
            if i % 2 != n % 2 {
                for j in 0..n {
                    moves[k] = board[i][j];
                    k += 1;
                }
            } else {
                for j in (0..n).rev() {
                    moves[k] = board[i][j];
                    k += 1;
                }
            }
        }
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut visited: Vec<usize> = vec![0; n * n];
        visited[0] = 1;
        queue.push_back(0);
        while let Some(i) = queue.pop_front() {
            if i == n * n - 1 {
                return visited[i] as i32 - 1;
            } else {
                for j in i + 1..=i + 6 {
                    if j >= n * n {
                        break;
                    }
                    let k = if moves[j] == -1 {
                        j
                    } else {
                        (moves[j] - 1) as usize
                    };
                    if visited[k] == 0 {
                        visited[k] = visited[i] + 1;
                        queue.push_back(k);
                    }
                }
            }
        }
        -1
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
        Solution::snakes_and_ladders(grid![
            [-1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1],
            [-1, 35, -1, -1, 13, -1],
            [-1, -1, -1, -1, -1, -1],
            [-1, 15, -1, -1, -1, -1]
        ])
    );
}

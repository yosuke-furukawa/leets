use std::collections::VecDeque;

impl Solution {
    pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        if board[click[0] as usize][click[1] as usize] == 'M' {
            board[click[0] as usize][click[1] as usize] = 'X';
            return board;
        }
        let r = board.len() as i32;
        let c = board[0].len() as i32;
        let dirs = [
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];
        let mut q = VecDeque::new();
        q.push_back((click[0], click[1]));

        while !q.is_empty() {
            let (i, j) = q.pop_front().unwrap();

            if board[i as usize][j as usize] == 'E' {
                let mut mines = 0;
                let mut neighbors = VecDeque::new();
                for (di, dj) in &dirs {
                    let ni = i + di;
                    let nj = j + dj;
                    if ni >= 0 && ni < r && nj >= 0 && nj < c {
                        if board[ni as usize][nj as usize] == 'M' {
                            mines += 1;
                        } else if board[ni as usize][nj as usize] == 'E' {
                            neighbors.push_back((ni, nj));
                        }
                    }
                }
                if mines > 0 {
                    board[i as usize][j as usize] = (b'0' + mines) as char;
                } else {
                    board[i as usize][j as usize] = 'B';
                    q.append(&mut neighbors);
                }
            }
        }

        board
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::update_board(
            vec![
                vec!['E', 'E', 'E', 'E', 'E'],
                vec!['E', 'E', 'M', 'E', 'E'],
                vec!['E', 'E', 'E', 'E', 'E'],
                vec!['E', 'E', 'E', 'E', 'E']
            ],
            vec![3, 0]
        )
    );
}

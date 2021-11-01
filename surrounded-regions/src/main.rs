use std::collections::VecDeque;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() {
            return;
        }

        let ws = 0;
        let we = board[0].len() - 1;
        let hs = 0;
        let he = board.len() - 1;
        let mut queue = VecDeque::new();
        for w in &[ws, we] {
            for h in hs..=he {
                match board[h][*w] {
                    'O' => {
                        board[h][*w] = '#';
                        queue.push_back((h, *w));
                    }
                    _ => continue,
                }
            }
        }

        for h in &[hs, he] {
            for w in ws..=we {
                match board[*h][w] {
                    'O' => {
                        board[*h][w] = '#';
                        queue.push_back((*h, w));
                    }
                    _ => continue,
                }
            }
        }

        while let Some((h, w)) = queue.pop_back() {
            let north = if h >= 1 { board[h - 1][w] } else { 'X' };
            let east = if w + 1 < board[h].len() {
                board[h][w + 1]
            } else {
                'X'
            };
            let west = if w >= 1 { board[h][w - 1] } else { 'X' };
            let south = if h + 1 < board.len() {
                board[h + 1][w]
            } else {
                'X'
            };
            if north == 'O' {
                board[h - 1][w] = '#';
                queue.push_back((h - 1, w));
            }
            if east == 'O' {
                board[h][w + 1] = '#';
                queue.push_back((h, w + 1));
            }
            if west == 'O' {
                board[h][w - 1] = '#';
                queue.push_back((h, w - 1));
            }
            if south == 'O' {
                board[h + 1][w] = '#';
                queue.push_back((h + 1, w));
            }
        }

        for h in hs..=he {
            for w in ws..=we {
                if board[h][w] == '#' {
                    board[h][w] = 'O';
                } else {
                    board[h][w] = 'X';
                }
            }
        }
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
    let mut board = grid![
        ['X', 'X', 'X', 'X'],
        ['X', 'O', 'O', 'X'],
        ['X', 'X', 'O', 'X'],
        ['X', 'O', 'X', 'X']
    ];
    Solution::solve(&mut board);
    println!("{:?}", board);
}

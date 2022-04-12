impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let will0 = -1;
        let will1 = -2;
        for c in 0..board.len() {
            for r in 0..board[0].len() {
                let target = board[c][r];
                let mut neighbors = vec![];
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        let nc = c as i32 + i;
                        let nr = r as i32 + j;
                        if nc < 0
                            || nc >= board.len() as i32
                            || nr < 0
                            || nr >= board[0].len() as i32
                        {
                            continue;
                        }
                        neighbors.push(board[nc as usize][nr as usize]);
                    }
                }
                let alive_cells = neighbors.iter().filter(|&&x| x == 1 || x == will0).count();
                if target == 0 {
                    if alive_cells == 3 {
                        board[c][r] = will1;
                    }
                } else if !(2..=3).contains(&alive_cells) {
                    board[c][r] = will0;
                }
            }
        }
        for c in 0..board.len() {
            for r in 0..board[0].len() {
                if board[c][r] == will1 {
                    board[c][r] = 1;
                } else if board[c][r] == will0 {
                    board[c][r] = 0;
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
    let mut board = grid!([0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]);
    Solution::game_of_life(&mut board);
    println!("{:?}", board);
}

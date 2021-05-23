impl Solution {
    fn is_valid(board: &[Vec<i32>], row: i32, col: i32, n: i32) -> bool {
        for r in 0..row {
            if board[r as usize][col as usize] == 1 {
                return false;
            }
        }

        for c in 0..col {
            if board[row as usize][c as usize] == 1 {
                return false;
            }
        }

        for i in 0..n {
            if row + i < n && col + i < n && board[(row + i) as usize][(col + i) as usize] == 1 {
                return false;
            }
            if row - i >= 0 && col - i >= 0 && board[(row - i) as usize][(col - i) as usize] == 1 {
                return false;
            }
            if row + i < n && col - i >= 0 && board[(row + i) as usize][(col - i) as usize] == 1 {
                return false;
            }
            if row - i >= 0 && col + i < n && board[(row - i) as usize][(col + i) as usize] == 1 {
                return false;
            }
        }

        true
    }
    fn backtrack(board: &mut Box<Vec<Vec<i32>>>, row: i32, n: i32, answers: &mut Vec<Vec<String>>) {
        for col in 0..n {
            if Self::is_valid(board.as_ref(), row, col, n) {
                *board
                    .get_mut(row as usize)
                    .unwrap()
                    .get_mut(col as usize)
                    .unwrap() = 1;
                if row + 1 == n {
                    let mut answer = vec![];
                    for b in board.as_ref() {
                        let mut str = "".to_string();
                        for v in b {
                            str += match v {
                                1 => "Q",
                                _ => ".",
                            };
                        }
                        answer.push(str);
                    }
                    answers.push(answer);
                } else {
                    Self::backtrack(board, row + 1, n, answers);
                }
                board[row as usize][col as usize] = 0;
            }
        }
    }
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let board = vec![vec![0; n as usize]; n as usize];
        let mut answer = vec![];
        Self::backtrack(&mut Box::new(board), 0, n, &mut answer);
        answer
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::solve_n_queens(4));
}

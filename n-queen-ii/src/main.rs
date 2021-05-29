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
    fn backtrack(board: &mut Box<Vec<Vec<i32>>>, row: i32, n: i32, count: &mut i32) -> i32 {
        for col in 0..n {
            if Self::is_valid(board.as_ref(), row, col, n) {
                *board
                    .get_mut(row as usize)
                    .unwrap()
                    .get_mut(col as usize)
                    .unwrap() = 1;
                if row + 1 == n {
                    *count += 1;
                } else {
                    *count = Self::backtrack(board, row + 1, n, count);
                }
                board[row as usize][col as usize] = 0;
            }
        }
        *count
    }
    pub fn total_n_queens(n: i32) -> i32 {
        let board = vec![vec![0; n as usize]; n as usize];
        let mut answer = 0;
        Self::backtrack(&mut Box::new(board), 0, n, &mut answer);
        answer
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::total_n_queens(4));
    println!("{}", Solution::total_n_queens(1));
}

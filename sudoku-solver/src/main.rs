const NUMS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

impl Solution {
    fn find_unassigned(board: &Vec<Vec<char>>, n: usize) -> (usize, usize, bool) {
        for nx in 0..n {
            for ny in 0..n {
                if board[nx][ny] == '.' {
                    return (nx, ny, true);
                }
            }
        }
        (0, 0, false)
    }
    fn is_valid(board: &Vec<Vec<char>>, num: char, x: usize, y: usize, n: usize) -> bool {
        for xi in 0..n {
            if board[xi][y] == num {
                return false;
            }
        }

        for yi in 0..n {
            if board[x][yi] == num {
                return false;
            }
        }

        let box_x = x / 3;
        let box_y = y / 3;

        for xi in box_x * 3..box_x * 3 + 3 {
            for yi in box_y * 3..box_y * 3 + 3 {
                if board[xi][yi] == num {
                    return false;
                }
            }
        }

        true
    }
    fn solve(board: &mut Vec<Vec<char>>) -> bool {
        let n = board.len();
        let (x, y, unassigned) = Self::find_unassigned(board, n);
        if !unassigned {
            return true;
        }
        for num in NUMS.iter() {
            if Self::is_valid(board, *num, x, y, n) {
                board[x][y] = *num;
                if Self::solve(board) {
                    return true;
                }
                board[x][y] = '.';
            }
        }
        false
    }
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve(board);
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
    let mut sudoku = grid![
        ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ];
    Solution::solve_sudoku(&mut sudoku);
    println!("{:?}", sudoku);
}

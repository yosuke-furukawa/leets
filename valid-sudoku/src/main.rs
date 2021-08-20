impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let n = 9;
        let mut rows = vec![0; n];
        let mut cols = vec![0; n];
        let mut boxes = vec![0; n];
        for r in 0..n {
            for c in 0..n {
                if board[r][c] == '.' {
                    continue;
                }
                let val = board[r][c] as u32 - '0' as u32;
                let pos = 1 << (val - 1);

                if (rows[r] & pos) > 0 {
                    return false;
                }
                rows[r] |= pos;

                if (cols[c] & pos) > 0 {
                    return false;
                }
                cols[c] |= pos;

                let idx = (r / 3) * 3 + c / 3;
                if (boxes[idx] & pos) > 0 {
                    return false;
                }
                boxes[idx] |= pos;
            }
        }
        true
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
        Solution::is_valid_sudoku(grid![
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ])
    );
}

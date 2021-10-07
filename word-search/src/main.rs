impl Solution {
    fn backtrack(
        board: &mut Vec<Vec<char>>,
        row: i32,
        col: i32,
        word: &Vec<char>,
        index: usize,
    ) -> bool {
        if index >= word.len() {
            return true;
        }

        if row < 0
            || row == board.len() as i32
            || col < 0
            || col == board[0].len() as i32
            || board[row as usize][col as usize] != word[index]
        {
            return false;
        }
        let mut result = false;
        board[row as usize][col as usize] = '#';
        let row_offsets = [0, 1, 0, -1];
        let col_offsets = [1, 0, -1, 0];
        for d in 0..4 {
            result = Self::backtrack(
                board,
                row + row_offsets[d],
                col + col_offsets[d],
                word,
                index + 1,
            );
            if result {
                break;
            }
        }
        board[row as usize][col as usize] = word[index];
        result
    }
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        for row in 0..board.len() {
            for col in 0..board[0].len() {
                if Self::backtrack(
                    &mut board,
                    row as i32,
                    col as i32,
                    &word.chars().collect(),
                    0,
                ) {
                    return true;
                }
            }
        }
        false
    }
}

struct Solution;

macro_rules! grid {
    ( $([$( $x:expr ),*]),* ) => {
        {
            vec![$(vec![$($x), *],)*]
        }
    };
}

fn main() {
    println!(
        "{}",
        Solution::exist(
            grid![
                ['A', 'B', 'C', 'E'],
                ['S', 'F', 'C', 'S'],
                ['A', 'D', 'E', 'E']
            ],
            "ABCCED".to_string()
        )
    );
}

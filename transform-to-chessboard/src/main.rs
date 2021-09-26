impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let first_row = board.get(0).unwrap();
        let first_col = board
            .iter()
            .map(|row| *row.get(0).unwrap())
            .collect::<Vec<i32>>();
        if !check_single_row(first_row) || !check_single_row(&first_col) {
            return -1;
        }
        for i in 1..n {
            let row = board.get(i).unwrap();
            if first_row[0] == row[0] {
                if !row.iter().zip(first_row.iter()).all(|(&a, &b)| a == b) {
                    return -1;
                }
            } else {
                if !row.iter().zip(first_row.iter()).all(|(&a, &b)| a ^ b == 1) {
                    return -1;
                }
            }
        }
        for i in 1..n {
            let col = board
                .iter()
                .map(|row| *row.get(i).unwrap())
                .collect::<Vec<i32>>();
            if first_col[0] == col[0] {
                if !col.iter().zip(first_col.iter()).all(|(&a, &b)| a == b) {
                    return -1;
                }
            } else {
                if !col.iter().zip(first_col.iter()).all(|(&a, &b)| a + b == 1) {
                    return -1;
                }
            }
        }
        count(first_row) + count(&first_col)
    }
}

fn check_single_row(row: &[i32]) -> bool {
    row.len() / 2 <= row.iter().filter(|&&i| i == 0).count()
        && row.iter().filter(|&&i| i == 0).count() <= row.len() / 2 + 1
}

fn count(row: &[i32]) -> i32 {
    let len = row.len();
    return if len % 2 == 0 {
        let count0 = row
            .iter()
            .enumerate()
            .filter(|&(idx, &i)| idx % 2 == 0 && i != 0)
            .count() as i32;
        let count1 = row
            .iter()
            .enumerate()
            .filter(|&(idx, &i)| idx % 2 == 0 && i != 1)
            .count() as i32;
        count0.min(count1)
    } else {
        if row.iter().filter(|&&i| i == 0).count() > len / 2 {
            row.iter()
                .enumerate()
                .filter(|&(idx, &i)| idx % 2 == 0 && i != 0)
                .count() as i32
        } else {
            row.iter()
                .enumerate()
                .filter(|&(idx, &i)| idx % 2 == 0 && i != 1)
                .count() as i32
        }
    };
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
        Solution::moves_to_chessboard(grid![
            [0, 1, 1, 0],
            [0, 1, 1, 0],
            [1, 0, 0, 1],
            [1, 0, 0, 1]
        ])
    );
}

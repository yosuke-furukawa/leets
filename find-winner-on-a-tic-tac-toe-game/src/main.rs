impl Solution {
    fn all(matrix: &Vec<Vec<i32>>, target: i32) -> bool {
        for x in 0..3 {
            if matrix[x][0] == matrix[x][1]
                && matrix[x][1] == matrix[x][2]
                && matrix[x][0] == target
            {
                return true;
            }
        }
        for y in 0..3 {
            if matrix[0][y] == matrix[1][y]
                && matrix[1][y] == matrix[2][y]
                && matrix[0][y] == target
            {
                return true;
            }
        }
        if matrix[0][0] == matrix[1][1] && matrix[1][1] == matrix[2][2] && matrix[0][0] == target {
            return true;
        }
        if matrix[2][0] == matrix[1][1] && matrix[1][1] == matrix[0][2] && matrix[2][0] == target {
            return true;
        }
        false
    }
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut matrix = vec![vec![0; 3]; 3];
        let mut v = 1;
        for m in moves.iter() {
            matrix[m[0] as usize][m[1] as usize] = v;
            v *= -1;
        }
        if Self::all(&matrix, 1) {
            "A".to_string()
        } else if Self::all(&matrix, -1) {
            "B".to_string()
        } else if moves.len() == 9 {
            "Draw".to_string()
        } else {
            "Pending".to_string()
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
    println!(
        "{}",
        Solution::tictactoe(grid![[0, 0], [2, 0], [1, 1], [2, 1], [2, 2]])
    );
}

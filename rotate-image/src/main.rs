impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for i in 0..matrix.len() {
            for j in i..matrix.len() {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        for row in matrix.iter_mut() {
            row.reverse();
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
    let mut image = grid![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    Solution::rotate(&mut image);
    println!("{:?}", image);
}

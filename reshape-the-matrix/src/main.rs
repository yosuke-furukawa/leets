impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let mut array = vec![];
        for m in mat.iter() {
            for n in m {
                array.push(*n);
            }
        }
        if r * c != array.len() as i32 {
            return mat;
        }
        let mut new_matrix = vec![vec![0; c as usize]; r as usize];

        for (i, num) in array.iter().enumerate() {
            new_matrix[i / c as usize][i % c as usize] = *num;
        }
        new_matrix
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
        "{:?}",
        Solution::matrix_reshape(grid![[1, 2], [3, 4]], 1, 4)
    );
    println!(
        "{:?}",
        Solution::matrix_reshape(grid![[1, 2], [3, 4]], 2, 4)
    );
    println!("{:?}", Solution::matrix_reshape(grid![[1, 2]], 1, 1));
}

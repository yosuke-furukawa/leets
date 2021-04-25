use std::collections::HashSet;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let height = matrix.len();
        let width = matrix[0].len();
        for m in matrix.iter_mut().take(height) {
            m.reverse();
        }

        let mut set = HashSet::new();
        for y in 0..height {
            for x in 0..width {
                let nx = height - y - 1;
                let ny = width - x - 1;

                if set.contains(&(nx, ny)) || set.contains(&(x, y)) {
                    continue;
                }
                let temp = matrix[y][x];
                matrix[y][x] = matrix[ny][nx];
                matrix[ny][nx] = temp;
                set.insert((x, y));
                set.insert((nx, ny));
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
    let mut image = grid![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    Solution::rotate(&mut image);
    println!("{:?}", image);
}

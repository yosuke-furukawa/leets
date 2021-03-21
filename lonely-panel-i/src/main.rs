use std::collections::HashSet;

impl Solution {
    pub fn find_lonely_pixel(picture: Vec<Vec<char>>) -> i32 {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        let mut count = 0;
        for row in 0..picture.len() {
            for col in 0..picture[row].len() {
                if picture[row][col] == 'B' {
                    visited.insert((row, col));
                }
            }
        }

        for (row, col) in visited.iter() {
            let mut lonely = true;
            for row2 in 0..picture.len() {
                if picture[row2][*col] == 'B' && row2 != *row {
                    lonely = false;
                    break;
                }
            }
            if !lonely {
                continue;
            }
            for col2 in 0..picture[0].len() {
                if picture[*row][col2] == 'B' && col2 != *col {
                    lonely = false;
                    break;
                }
            }
            if !lonely {
                continue;
            }
            count += 1;
        }
        count
    }
}

struct Solution;

#[macro_export]
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
        Solution::find_lonely_pixel(grid![['W', 'W', 'B'], ['W', 'B', 'W'], ['B', 'W', 'W']])
    );
}

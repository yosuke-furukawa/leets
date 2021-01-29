impl Solution {
    pub fn count_corner_rectangles(grid: Vec<Vec<i32>>) -> i32 {
        let cols = grid.len();
        if cols < 1 {
            return 0;
        }
        let rows = grid[0].len();

        let mut count = 0;
        for col in 1..cols {
            for i in 0..rows {
                for j in i + 1..rows {
                    if grid[col][i] == 1 && grid[col][j] == 1 {
                        let mut c = col;
                        while c >= 1 {
                            c -= 1;

                            if grid[c][i] == 1 && grid[c][j] == 1 {
                                count += 1;
                            }
                        }
                    }
                }
            }
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
        Solution::count_corner_rectangles(grid![
            [1, 0, 0, 1, 0],
            [0, 0, 1, 0, 1],
            [0, 0, 0, 1, 0],
            [1, 0, 1, 0, 1]
        ])
    );
    println!(
        "{}",
        Solution::count_corner_rectangles(grid![[1, 1, 1], [1, 1, 1], [1, 1, 1]])
    );
    println!("{}", Solution::count_corner_rectangles(grid![[1, 1, 1, 1]]));
}

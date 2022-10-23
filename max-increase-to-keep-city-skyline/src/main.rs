impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut r_max = vec![0; grid.len()];
        let mut c_max = vec![0; grid[0].len()];
        for (i, row) in grid.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                r_max[i] = r_max[i].max(c);
                c_max[j] = c_max[j].max(c);
            }
        }
        for (i, row) in grid.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                res += r_max[i].min(c_max[j]) - c;
            }
        }
        res
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
        Solution::max_increase_keeping_skyline(grid![
            [3, 0, 8, 4],
            [2, 4, 5, 7],
            [9, 2, 6, 3],
            [0, 3, 1, 0]
        ])
    );
}

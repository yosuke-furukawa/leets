impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut zeros = 0;
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if grid[x][y] == 1 {
                    let north = if x > 0 { grid[x - 1][y] } else { 0 };
                    let west = if y > 0 { grid[x][y - 1] } else { 0 };
                    let east = if y < grid[x].len() - 1 {
                        grid[x][y + 1]
                    } else {
                        0
                    };
                    let south = if x < grid.len() - 1 {
                        grid[x + 1][y]
                    } else {
                        0
                    };
                    zeros += 4 - (north + west + east + south);
                }
            }
        }
        zeros
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
        Solution::island_perimeter(grid![
            [0, 1, 0, 0],
            [1, 1, 1, 0],
            [0, 1, 0, 0],
            [1, 1, 0, 0]
        ])
    );
    println!("{}", Solution::island_perimeter(grid![[1, 1]]));
}

impl Solution {
    fn go_through(path: &[i32], x: i32) -> i32 {
        let i = x as usize;

        if path[i] == 1 && i + 1 < path.len() && path[i + 1] == -1 {
            return -1;
        }
        if path[i] == -1 && i >= 1 && path[i - 1] == 1 {
            return -1;
        }
        if path[i] == 1 {
            x + 1
        } else {
            x - 1
        }
    }
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        for ball in 0..grid[0].len() {
            let mut ball_x = ball as i32;
            let mut ball_y = 0;
            for col in grid.iter() {
                ball_x = Self::go_through(col, ball_x);
                if ball_x == -1 {
                    break;
                }
                if ball_x as usize >= col.len() {
                    break;
                }
                ball_y += 1;
            }
            if ball_y == grid.len() {
                res.push(ball_x);
            } else {
                res.push(-1);
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
        "{:?}",
        Solution::find_ball(grid![
            [1, 1, 1, -1, -1],
            [1, 1, 1, -1, -1],
            [-1, -1, -1, 1, 1],
            [1, 1, 1, 1, -1],
            [-1, -1, -1, -1, -1]
        ])
    );
    println!(
        "{:?}",
        Solution::find_ball(grid![
            [1, 1, 1, 1, 1, 1],
            [-1, -1, -1, -1, -1, -1],
            [1, 1, 1, 1, 1, 1],
            [-1, -1, -1, -1, -1, -1]
        ])
    );
}

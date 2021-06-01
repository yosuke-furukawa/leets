impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; 3]; costs.len() + 1];
        for (i, cost) in costs.iter().enumerate() {
            for (j, c) in cost.iter().enumerate() {
                dp[i + 1][j] = match j {
                    0 => dp[i][1].min(dp[i][2]) + c,
                    1 => dp[i][0].min(dp[i][2]) + c,
                    _ => dp[i][0].min(dp[i][1]) + c,
                };
            }
        }
        *dp.last().unwrap().iter().min().unwrap()
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
        Solution::min_cost(grid![[17, 2, 17], [16, 16, 5], [14, 3, 19]])
    );
    println!("{}", Solution::min_cost(grid![[7, 6, 2]]));
}

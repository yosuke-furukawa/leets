impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let mut dp: Vec<_> = dungeon
            .last()
            .unwrap()
            .iter()
            .rev()
            .scan(1, |s, x| {
                *s = (*s - x).max(1);
                Some(*s)
            })
            .collect();
        for row in dungeon[..dungeon.len() - 1].iter().rev() {
            for (i, x) in row.iter().rev().enumerate() {
                if i > 0 {
                    dp[i] = dp[i].min(dp[i - 1]);
                }
                dp[i] = (dp[i] - x).max(1);
            }
        }
        dp[dp.len() - 1]
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
        Solution::calculate_minimum_hp(grid![[-2, -3, 3], [-5, -10, 1], [10, 30, -5]])
    );
    println!("{}", Solution::calculate_minimum_hp(grid![[0]]));
    println!("{}", Solution::calculate_minimum_hp(grid![[-200]]));
}

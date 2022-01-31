impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|account| account.iter().sum::<i32>())
            .max()
            .unwrap()
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
    println!("{}", Solution::maximum_wealth(grid![[1, 2, 3], [3, 2, 1]]));
    println!(
        "{}",
        Solution::maximum_wealth(grid![[1, 5], [7, 3], [3, 5]])
    );
    println!(
        "{}",
        Solution::maximum_wealth(grid![[2, 8, 7], [7, 1, 3], [1, 9, 5]])
    );
}

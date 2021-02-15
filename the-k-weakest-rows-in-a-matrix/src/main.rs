impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut rows: Vec<(usize, i32)> = mat
            .iter()
            .enumerate()
            .map(|(i, nums)| (i, nums.iter().sum()))
            .collect();
        rows.sort_by(|a, b| a.1.cmp(&b.1));
        rows.iter().take(k as usize).map(|v| v.0 as i32).collect()
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
        "{:?}",
        Solution::k_weakest_rows(
            grid![
                [1, 1, 0, 0, 0],
                [1, 1, 1, 1, 0],
                [1, 0, 0, 0, 0],
                [1, 1, 0, 0, 0],
                [1, 1, 1, 1, 1]
            ],
            3
        )
    );
}

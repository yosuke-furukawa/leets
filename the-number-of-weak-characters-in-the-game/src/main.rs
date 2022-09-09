impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut properties = properties;
        properties.sort_unstable_by_key(|property| (-property[0], property[1]));
        let mut max_def = 0;
        let mut ans = 0;
        for property in properties {
            if property[1] < max_def {
                ans += 1;
            } else {
                max_def = max_def.max(property[1]);
            }
        }
        ans
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
        Solution::number_of_weak_characters(grid![[5, 5], [6, 3], [3, 6]])
    );
    println!(
        "{}",
        Solution::number_of_weak_characters(grid![[2, 2], [3, 3]])
    );
    println!(
        "{}",
        Solution::number_of_weak_characters(grid![[1, 5], [10, 4], [4, 3]])
    );
    println!(
        "{}",
        Solution::number_of_weak_characters(grid![
            [7, 7],
            [1, 2],
            [9, 7],
            [7, 3],
            [3, 10],
            [9, 8],
            [8, 10],
            [4, 3],
            [1, 5],
            [1, 5]
        ])
    );
    println!(
        "{}",
        Solution::number_of_weak_characters(grid![[1, 1], [2, 1], [2, 2], [1, 2]])
    );
}

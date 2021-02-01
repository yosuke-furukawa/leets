impl Solution {
    pub fn min_distance(
        height: i32,
        width: i32,
        tree: Vec<i32>,
        squirrel: Vec<i32>,
        nuts: Vec<Vec<i32>>,
    ) -> i32 {
        let mut total_distance = 0;
        let mut max_tree_squirrel = -1000000000;
        fn distance(a: &[i32], b: &[i32]) -> i32 {
            (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
        };
        for nut in nuts {
            total_distance += distance(&nut, &tree) * 2;
            max_tree_squirrel =
                max_tree_squirrel.max(distance(&nut, &tree) - distance(&nut, &squirrel));
        }
        total_distance - max_tree_squirrel
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
        Solution::min_distance(5, 7, vec![2, 2], vec![4, 4], grid![[3, 0], [2, 5]])
    );
    println!(
        "{}",
        Solution::min_distance(1, 3, vec![0, 1], vec![0, 0], grid![[0, 2]])
    );
    println!(
        "{}",
        Solution::min_distance(
            1,
            3,
            vec![3, 2],
            vec![0, 1],
            grid![
                [2, 0],
                [4, 1],
                [0, 4],
                [1, 3],
                [1, 0],
                [3, 4],
                [3, 0],
                [2, 3],
                [0, 2],
                [0, 0],
                [2, 2],
                [4, 2],
                [3, 3],
                [4, 4],
                [4, 0],
                [4, 3],
                [3, 1],
                [2, 1],
                [1, 4],
                [2, 4]
            ]
        )
    );
}

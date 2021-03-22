use std::collections::HashSet;

impl Solution {
    pub fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
        let mut set: HashSet<i32> = mat.first().unwrap().iter().cloned().collect();
        for s in mat.iter().skip(1) {
            set = set
                .intersection(&s.iter().cloned().collect())
                .cloned()
                .collect();
        }
        if set.is_empty() {
            return -1;
        }
        let mut arr: Vec<i32> = set.iter().cloned().collect();
        arr.sort_unstable();
        *arr.first().unwrap()
    }
}

struct Solution;

macro_rules! grid {
    ( $([$( $x: expr ), *]), *) => {
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
        Solution::smallest_common_element(grid![
            [1, 2, 3, 4, 5],
            [2, 4, 5, 8, 10],
            [3, 5, 7, 9, 11],
            [1, 3, 5, 7, 9]
        ])
    );
}

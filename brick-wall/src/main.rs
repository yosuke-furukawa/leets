use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let mut map = HashMap::new();
        for w in wall.iter() {
            let mut len = 0;
            for n in w.iter().take(w.len() - 1) {
                len += n;
                let value = map.entry(len).or_insert(0);
                *value += 1;
                count = count.max(*value);
            }
        }
        wall.len() as i32 - count
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
        Solution::least_bricks(grid![
            [1, 2, 2, 1],
            [3, 1, 2],
            [1, 3, 2],
            [2, 4],
            [3, 1, 2],
            [1, 3, 1, 1]
        ])
    );
    println!("{}", Solution::least_bricks(grid![[1], [1], [1]]));
    println!("{}", Solution::least_bricks(grid![[1, 1], [2], [1, 1]]));
    println!("{}", Solution::least_bricks(grid![[1, 2, 3], [3, 2, 1]]));
}

use std::collections::HashMap;

impl Solution {
    fn gcd(mut x: i32, mut y: i32) -> i32 {
        while y != 0 {
            let t = x % y;
            x = y;
            y = t;
        }
        x
    }
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable_by_key(|p| (p[0], p[1]));
        let mut res = 0;
        for i in 0..points.len() {
            let mut map = HashMap::new();
            let mut same = 0;
            let mut max = 0;
            for j in i + 1..points.len() {
                let x = points[j][0] - points[i][0];
                let y = points[j][1] - points[i][1];
                if x == 0 && y == 0 {
                    same += 1;
                    continue;
                }
                let gcd = Self::gcd(x, y);
                let x = x / gcd;
                let y = y / gcd;
                let count = map.entry((x, y)).or_insert(0);
                *count += 1;
                max = max.max(*count);
            }
            res = res.max(max + same + 1);
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
    println!("{}", Solution::max_points(grid![[1, 1], [2, 2], [3, 3]]));
    println!(
        "{}",
        Solution::max_points(grid![[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]])
    );
}

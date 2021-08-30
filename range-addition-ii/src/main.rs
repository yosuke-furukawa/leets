impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut row = m;
        let mut col = n;
        for op in ops {
            row = row.min(op[0]);
            col = col.min(op[1]);
        }

        row * col
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
    println!("{}", Solution::max_count(3, 3, grid![[2, 2], [3, 3]]));
}

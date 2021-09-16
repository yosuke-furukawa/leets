impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return Vec::new();
        }
        let mut ans = Vec::with_capacity(matrix.len() * matrix[0].len());
        let mut r1 = 0;
        let mut r2 = matrix.len() as i32 - 1;
        let mut c1 = 0;
        let mut c2 = matrix[0].len() as i32 - 1;
        while r1 <= r2 && c1 <= c2 {
            for c in c1..=c2 {
                ans.push(matrix[r1 as usize][c as usize]);
            }
            for r in r1 + 1..=r2 {
                ans.push(matrix[r as usize][c2 as usize]);
            }
            if r1 < r2 && c1 < c2 {
                for c in (c1 + 1..=c2 - 1).rev() {
                    ans.push(matrix[r2 as usize][c as usize]);
                }
                for r in (r1 + 1..=r2).rev() {
                    ans.push(matrix[r as usize][c1 as usize]);
                }
            }
            r1 += 1;
            r2 -= 1;
            c1 += 1;
            c2 -= 1;
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
        "{:?}",
        Solution::spiral_order(grid![[1, 2, 3], [4, 5, 6], [7, 8, 9]])
    );
    println!("{:?}", Solution::spiral_order(grid![[1]]));
}

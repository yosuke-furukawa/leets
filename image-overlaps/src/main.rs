impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let n = img1.len() as i32;
        for i in -n..=n {
            for j in -n..=n {
                let mut temp = 0;
                for k in 0..n {
                    let r1 = k + i;
                    if r1 < 0 || n <= r1 {
                        continue;
                    }
                    for l in 0..n {
                        let c1 = l + j;
                        if c1 < 0 || n <= c1 {
                            continue;
                        }
                        let v1 = img1[r1 as usize][c1 as usize];
                        if v1 == 1 && v1 == img2[k as usize][l as usize] {
                            temp += 1;
                        }
                    }
                }
                result = std::cmp::max(result, temp);
            }
        }
        result
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
        Solution::largest_overlap(
            grid![[1, 1, 0], [0, 1, 0], [0, 1, 0]],
            grid![[0, 0, 0], [0, 1, 1], [0, 0, 1]],
        )
    );
    println!(
        "{}",
        Solution::largest_overlap(
            grid![
                [0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0]
            ],
            grid![
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0]
            ]
        )
    );
}

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid[0].len();
        let mut flat = grid.into_iter().flatten().collect::<Vec<i32>>();
        let len = flat.len();
        flat.rotate_right(k as usize % len);
        flat.chunks(m)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<i32>>>()
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
        Solution::shift_grid(grid![[1, 2, 3], [4, 5, 6], [7, 8, 9]], 1)
    );
    println!("{:?}", Solution::shift_grid(grid![[1]], 100));
}

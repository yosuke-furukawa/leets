impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut powers = vec![];
        for point in points {
            powers.push((point[0].pow(2) + point[1].pow(2), point));
        }
        powers.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        powers.into_iter().take(k as usize).map(|p| p.1).collect()
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
    println!("{:?}", Solution::k_closest(grid![[1, 3], [-2, 2]], 1));
    println!(
        "{:?}",
        Solution::k_closest(grid![[3, 3], [5, -1], [-2, 4]], 2)
    );
}

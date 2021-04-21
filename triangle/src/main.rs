impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut prev = vec![];
        for array in triangle {
            if prev.is_empty() {
                prev = array;
                continue;
            }
            let mut current = vec![0; array.len()];
            for (i, a) in array.into_iter().enumerate() {
                if i == 0 {
                    current[i] = a + prev[i];
                    continue;
                }
                if i >= current.len() - 1 {
                    current[i] = a + prev[i - 1];
                    continue;
                }
                current[i] = a + prev[i - 1].min(prev[i]);
            }
            prev = current;
        }
        *prev.iter().min().unwrap()
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
        Solution::minimum_total(grid![[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]])
    );
}

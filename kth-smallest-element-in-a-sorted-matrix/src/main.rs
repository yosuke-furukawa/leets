impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut array = vec![];
        for m in matrix.iter() {
            for num in m.iter() {
                array.push(*num);
            }
        }
        array.sort_unstable();
        array[k as usize - 1]
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
        Solution::kth_smallest(grid![[1, 5, 9], [10, 11, 13], [12, 13, 15]], 8)
    );
}

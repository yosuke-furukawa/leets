use std::collections::BinaryHeap;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for m in matrix.iter() {
            for num in m.iter() {
                heap.push(num);
            }
        }
        while heap.len() > k as usize {
            heap.pop();
        }
        *heap.pop().unwrap()
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
    println!("{}", Solution::kth_smallest(grid![[1, 2], [1, 3]], 2));
}

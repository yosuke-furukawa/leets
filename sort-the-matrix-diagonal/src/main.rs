use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; mat[0].len()]; mat.len()];
        let mut map: HashMap<i32, BinaryHeap<i32>> = HashMap::new();
        let m = mat.len();
        let n = mat[0].len();
        for i in 0..m {
            for j in 0..n {
                map.entry(i as i32 - j as i32).or_default().push(-mat[i][j]);
            }
        }
        for i in 0..m {
            for j in 0..n {
                result[i][j] = -map.entry(i as i32 - j as i32).or_default().pop().unwrap();
            }
        }

        result
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::diagonal_sort(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]])
    );
}

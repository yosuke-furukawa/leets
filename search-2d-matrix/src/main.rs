impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        matrix.concat().binary_search(&target).is_ok()
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        )
    );
}

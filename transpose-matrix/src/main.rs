impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for rows in matrix {
            for (i, val) in rows.iter().enumerate() {
                if result.len() <= i {
                    result.push(vec![]);
                }
                result[i].push(*val);
            }
        }
        result
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
    println!(
        "{:?}",
        Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]])
    );
}

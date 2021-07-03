impl Solution {
    pub fn binary_search(matrix: &[i32], target: i32) -> usize {
        let mut left = 0usize;
        let mut right = matrix.len();
        while left < right {
            let mid = left + (right - left) / 2;
            match mid {
                _ if matrix[mid] == target => return mid,
                _ if matrix[mid] < target => left = mid + 1,
                _ => right = mid,
            }
        }
        left
    }

    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let mut max = std::i32::MIN;
        let row = matrix.len();
        let col = matrix[0].len();
        for left in 0..col {
            let mut sum: Vec<i32> = vec![0; row];
            for right in left..col {
                for idx in 0..row {
                    sum[idx] += matrix[idx][right];
                }
                let mut current_sum = 0;
                let mut temp: Vec<i32> = vec![0];
                for n in sum.iter().take(row) {
                    current_sum += n;
                    let ix = Solution::binary_search(&temp, current_sum - k);
                    if ix < temp.len() {
                        max = max.max(current_sum - temp[ix]);
                    }
                    let ix = Solution::binary_search(&temp, current_sum);
                    temp.insert(ix, current_sum);
                }
            }
        }
        max
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
        Solution::max_sum_submatrix(grid![[1, 0, 1], [0, -2, 3]], 2)
    );
    println!("{}", Solution::max_sum_submatrix(grid![[2, 4, -2]], 3));
}

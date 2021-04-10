impl Solution {
    fn dfs(matrix: &Vec<Vec<i32>>, i: i32, j:i32, m:i32, n:i32, cache: &mut Vec<Vec<i32>>) -> i32 {
        if cache[i as usize][j as usize] != 0 {
            return cache[i as usize][j as usize];
        }
        let mut max = 1;
        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let x = i + dir.0;
            let y = j + dir.1;
            if x < 0 || x >= m || y < 0 || y >= n || matrix[x as usize][y as usize] <= matrix[i as usize][j as usize] {
                continue;
            }
            let len = 1 + Self::dfs(matrix, x, y, m, n, cache);
            max = max.max(len);
        }
        cache[i as usize][j as usize] = max;
        max
    }
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let mut max = 1;
        let m = matrix.len();
        let n = matrix[0].len();
        let mut cache = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let len = Self::dfs(&matrix, i as i32, j as i32, m as i32, n as i32, &mut cache);
                max = max.max(len);
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
    println!("{}", Solution::longest_increasing_path(grid![[9,9,4],[6,6,8],[2,1,1]]));
}

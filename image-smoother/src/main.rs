impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let dirs = [
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (0, 0),
        ];
        let (n, m) = (img.len() as i32, img[0].len() as i32);
        let mut res = vec![vec![0; m as usize]; n as usize];
        for i in 0..n {
            for j in 0..m {
                let (mut num, mut sum) = (0, 0);
                for (new_i, new_j) in dirs
                    .iter()
                    .map(|&(di, dj)| (i + di, j + dj))
                    .filter(|&(new_i, new_j)| new_i >= 0 && new_i < n && new_j >= 0 && new_j < m)
                {
                    num += 1;
                    sum += img[new_i as usize][new_j as usize];
                }
                res[i as usize][j as usize] = sum / num;
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]])
    );
}

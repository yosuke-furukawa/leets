impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut count: Vec<Vec<usize>> = vec![vec![0; 3]; 6];
        let n = tops.len();
        for i in 0..n {
            let x = (tops[i] - 1) as usize;
            let y = (bottoms[i] - 1) as usize;
            count[x][0] += 1;
            count[y][1] += 1;
            if x == y {
                count[x][2] += 1;
            }
        }
        for i in 0..6 {
            if count[i][0] + count[i][1] - count[i][2] >= n {
                return (count[i][0].min(count[i][1]) - count[i][2]) as i32;
            }
        }
        -1
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2])
    );
}

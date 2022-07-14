use std::collections::HashMap;

impl Solution {
    fn distance(p1: &[i32], p2: &[i32]) -> i32 {
        (p2[0] - p1[0]).pow(2) + (p2[1] - p1[1]).pow(2)
    }
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut len_map = HashMap::new();
        for i in 0..points.len() {
            for j in 0..points.len() {
                if i == j {
                    continue;
                }
                let len = Solution::distance(&points[i], &points[j]);
                *len_map.entry(len).or_default() += 1;
            }
            res += len_map.values().map(|v| v * (v - 1)).sum::<i32>();
            len_map.clear();
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]])
    );
    println!(
        "{}",
        Solution::number_of_boomerangs(vec![
            vec![0, 0],
            vec![1, 0],
            vec![-1, 0],
            vec![0, 1],
            vec![0, -1]
        ])
    );
}

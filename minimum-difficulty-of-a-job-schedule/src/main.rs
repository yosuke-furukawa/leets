use std::collections::HashMap;

impl Solution {
    fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        let d = d as usize;
        if d > n {
            return -1;
        }
        let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
        Self::dp(0, d, &mut memo, &job_difficulty, n)
    }

    fn dp(
        start: usize,
        d: usize,
        memo: &mut HashMap<(usize, usize), i32>,
        jobs: &[i32],
        n: usize,
    ) -> i32 {
        if let Some(&res) = memo.get(&(start, d)) {
            return res;
        }
        let res = if d == 1 {
            *jobs[start..n].iter().max().unwrap()
        } else {
            if start + d == n {
                jobs[start..start + d].iter().sum()
            } else {
                let mut min = std::i32::MAX;
                let mut max = 0;
                for i in start..=(n - d) {
                    max = max.max(jobs[i]);
                    min = min.min(max + Self::dp(i + 1, d - 1, memo, jobs, n));
                }
                min
            }
        };
        memo.insert((start, d), res);
        res
    }
}


struct Solution;

fn main() {
    println!("{}", Solution::min_difficulty(vec![6, 5, 4, 3, 2, 1], 2));
}

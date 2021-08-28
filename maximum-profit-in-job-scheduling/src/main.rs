use std::collections::BTreeMap;

impl Solution {
    fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = vec![];
        let n = start_time.len();
        for i in 0..n {
            jobs.push((start_time[i], end_time[i], profit[i]));
        }
        jobs.sort_unstable();
        let mut memo: BTreeMap<i32, i32> = BTreeMap::new();
        let mut res = 0;
        for i in (0..n).rev() {
            let mut cur = jobs[i].2;
            if let Some((_, val)) = memo.range(jobs[i].1..).next() {
                cur += val;
            }
            res = res.max(cur);
            memo.insert(jobs[i].0, res);
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70])
    );
}

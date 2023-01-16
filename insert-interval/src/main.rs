impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut i = 0;
        let n = intervals.len();
        while i < n && intervals[i][1] < new_interval[0] {
            res.push(intervals[i].clone());
            i += 1;
        }
        let mut new_interval = new_interval;
        while i < n && intervals[i][0] <= new_interval[1] {
            new_interval[0] = new_interval[0].min(intervals[i][0]);
            new_interval[1] = new_interval[1].max(intervals[i][1]);
            i += 1;
        }
        res.push(new_interval);
        while i < n {
            res.push(intervals[i].clone());
            i += 1;
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
    );
}

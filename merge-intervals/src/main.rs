impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        let mut results = vec![];
        let mut target = intervals[0].clone();
        for interval in intervals {
            if target[1] < interval[0] {
                results.push(target.clone());
                target = interval;
            } else {
                target[1] = target[1].max(interval[1]);
            }
        }
        results.push(target.clone());
        results
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
        "{:?}",
        Solution::merge(grid![[1, 3], [2, 6], [8, 10], [15, 18]])
    );
}

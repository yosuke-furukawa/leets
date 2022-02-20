impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| {
            if a[0].cmp(&b[0]) == std::cmp::Ordering::Equal {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut res = 0;
        let mut prev_end = 0;
        for interval in intervals {
            let end = interval[1];
            if prev_end < end {
                res += 1;
                prev_end = end;
            }
        }
        res
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
        Solution::remove_covered_intervals(grid![[1, 4], [3, 6], [2, 8]])
    );
    println!(
        "{}",
        Solution::remove_covered_intervals(grid![[1, 2], [1, 4], [3, 4]])
    );
}

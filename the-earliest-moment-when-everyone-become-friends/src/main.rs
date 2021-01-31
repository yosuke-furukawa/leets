impl Solution {
    fn find(x: i32, parent: &[i32]) -> i32 {
        if x >= 0 && parent[x as usize] != x {
            return Solution::find(parent[x as usize], parent);
        }
        x
    }
    pub fn earliest_acq(logs: Vec<Vec<i32>>, n: i32) -> i32 {
        let mut logs = logs;
        logs.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut groups = Vec::with_capacity(n as usize);
        for i in 0..n {
            groups.push(i);
        }
        let mut count = n;
        for log in logs {
            let group1 = Solution::find(log[1], &groups);
            let group2 = Solution::find(log[2], &groups);
            if group1 != group2 {
                count -= 1;
                if count == 1 {
                    return log[0];
                }
                groups[group1 as usize] = group2;
            }
        }
        -1
    }
}

struct Solution;

#[macro_export]
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
        Solution::earliest_acq(
            grid![
                [20190101, 0, 1],
                [20190104, 3, 4],
                [20190107, 2, 3],
                [20190211, 1, 5],
                [20190224, 2, 4],
                [20190301, 0, 3],
                [20190312, 1, 2],
                [20190322, 4, 5]
            ],
            6
        )
    );
}

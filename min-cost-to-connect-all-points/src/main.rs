impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut cost = vec![i32::MAX; points.len()];
        cost[0] = 0;
        let mut prev = 0;
        let mut ans = 0;
        for _ in 0..(points.len() - 1) {
            let mut min = i32::MAX;
            let mut next = prev;
            for i in 0..points.len() {
                if cost[i] > 0 {
                    let dist = (points[prev][0] - points[i][0]).abs()
                        + (points[prev][1] - points[i][1]).abs();
                    cost[i] = cost[i].min(dist);
                    if cost[i] < min {
                        next = i;
                        min = cost[i];
                    }
                }
            }
            cost[next] = 0;
            ans += min;
            prev = next;
        }
        ans
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
        Solution::min_cost_connect_points(grid![[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]])
    );
}

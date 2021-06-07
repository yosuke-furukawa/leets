impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut f = vec![0; cost.len()];
        f[cost.len() - 1] = cost[cost.len() - 1];
        f[cost.len() - 2] = cost[cost.len() - 2];

        for i in (0..cost.len() - 2).rev() {
            f[i] = cost[i] + f[i + 1].min(f[i + 2]);
        }
        f[0].min(f[1])
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
    println!(
        "{}",
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
    );
}

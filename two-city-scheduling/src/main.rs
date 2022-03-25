impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut costs = costs;
        costs.sort_unstable_by(|a, b| (a[0] - a[1]).cmp(&(b[0] - b[1])));
        let mut res = 0;
        for i in 0..costs.len() / 2 {
            res += costs[i][0] + costs[i + costs.len() / 2][1];
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::two_city_sched_cost(vec![
            vec![10, 20],
            vec![30, 200],
            vec![400, 50],
            vec![30, 20]
        ])
    );
}

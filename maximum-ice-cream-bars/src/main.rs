impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut costs = costs;
        let mut coins = coins;
        costs.sort_unstable();
        let mut res = 0;
        for cost in costs {
            if coins <= 0 {
                break;
            }
            if coins >= cost {
                coins -= cost;
                res += 1;
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7));
}

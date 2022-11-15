use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut res = 0;
        let mut costs = VecDeque::from(costs);
        let mut fronts = BinaryHeap::new();
        for i in 0..candidates as usize {
            if let Some(cost) = costs.pop_front() {
                fronts.push(Reverse((cost, i)));
            }
        }
        let n = costs.len();
        let mut backs = BinaryHeap::new();
        for i in 0..candidates as usize {
            if let Some(cost) = costs.pop_back() {
                backs.push(Reverse((cost, n - i - 1)));
            }
        }
        for _ in 0..k {
            if fronts.is_empty() && backs.is_empty() {
                break;
            }
            let Reverse((front_cost, front_idx)) =
                fronts.pop().unwrap_or(Reverse((std::i32::MAX, 0)));
            let Reverse((back_cost, back_idx)) = backs.pop().unwrap_or(Reverse((std::i32::MAX, 0)));
            if front_cost <= back_cost {
                res += front_cost as i64;
                if let Some(cost) = costs.pop_front() {
                    fronts.push(Reverse((cost, front_idx)));
                }
                backs.push(Reverse((back_cost, back_idx)));
            } else {
                res += back_cost as i64;
                if let Some(cost) = costs.pop_back() {
                    backs.push(Reverse((cost, back_idx)));
                }
                fronts.push(Reverse((front_cost, front_idx)));
            }
        }
        res as i64
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4)
    );
    println!("{}", Solution::total_cost(vec![1, 2, 4, 1], 3, 3));
}

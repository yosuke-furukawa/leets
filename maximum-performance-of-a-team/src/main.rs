use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Eq, Debug)]
struct Worker {
    speed: u128,
    efficiency: u128,
}

impl Ord for Worker {
    fn cmp(&self, other: &Self) -> Ordering {
        self.speed.cmp(&other.speed)
    }
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Worker {
    fn eq(&self, other: &Self) -> bool {
        self.speed == other.speed && self.efficiency == other.efficiency
    }
}

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut workers = Vec::with_capacity(n as usize);
        for i in 0..n as usize {
            workers.push(Worker {
                speed: speed[i] as u128,
                efficiency: efficiency[i] as u128,
            });
        }
        workers.sort_unstable_by(|a, b| b.efficiency.cmp(&a.efficiency));
        let mut total_speed = 0;
        let mut max = 0;
        let mut heap = BinaryHeap::new();
        for worker in workers {
            total_speed += worker.speed;
            let efficiency = worker.efficiency;
            heap.push(Reverse(worker));
            if heap.len() > k as usize {
                if let Some(Reverse(w)) = heap.pop() {
                    total_speed -= w.speed;
                }
            }
            let total = total_speed * efficiency;
            max = max.max(total);
        }
        (max % 1_000_000_007) as i32
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2)
    );
}

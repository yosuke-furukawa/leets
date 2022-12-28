use std::collections::BinaryHeap;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from(piles);
        for _ in 0..k {
            if let Some(pile) = heap.pop() {
                let p = pile / 2;
                if pile % 2 == 1 {
                    heap.push(p + 1);
                } else {
                    heap.push(p);
                }
            }
        }
        heap.iter().sum()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_stone_sum(vec![5, 4, 9], 2));
}

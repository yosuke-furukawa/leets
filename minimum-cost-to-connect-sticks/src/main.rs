use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        if sticks.len() <= 1 {
            return 0;
        }
        for n in sticks {
            heap.push(Reverse(n));
        }
        let mut sticks = vec![];
        while heap.len() > 1 {
            let first = heap.pop();
            let second = heap.pop();
            if let Some(Reverse(f)) = first {
                if let Some(Reverse(s)) = second {
                    heap.push(Reverse(f + s));
                    sticks.push(f + s);
                }
            }
        }
        sticks.iter().fold(0, |sum, x| sum + *x)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::connect_sticks(vec![2, 4, 3]));
    println!("{}", Solution::connect_sticks(vec![1, 8, 3, 5]));
    println!("{}", Solution::connect_sticks(vec![5]));
}

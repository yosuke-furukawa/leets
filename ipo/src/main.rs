use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut projects = vec![];
        for i in 0..profits.len() {
            projects.push((capital[i], profits[i]));
        }
        projects.sort();
        let mut i = 0;
        let mut w = w;
        for _ in 0..k {
            while i < projects.len() && projects[i].0 <= w {
                heap.push(projects[i].1);
                i += 1;
            }
            if let Some(p) = heap.pop() {
                w += p;
            } else {
                break;
            }
        }
        w
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1])
    );
    println!(
        "{}",
        Solution::find_maximized_capital(1, 0, vec![1, 2, 3], vec![0, 1, 2])
    );
    println!(
        "{}",
        Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2])
    );
}

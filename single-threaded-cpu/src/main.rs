use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tasks = tasks
            .into_iter()
            .enumerate()
            .map(|(i, task)| (task[0], task[1], i))
            .collect::<Vec<_>>();
        tasks.sort_unstable();
        let mut res = Vec::with_capacity(tasks.len());
        let mut heap = BinaryHeap::new();
        let mut time = 0;
        let mut task = 0;
        while task < tasks.len() || !heap.is_empty() {
            if heap.is_empty() && time < tasks[task].0 {
                time = tasks[task].0 + tasks[task].1;
                res.push(tasks[task].2 as i32);
                task += 1;
            } else {
                let Reverse((duration, index)) = heap.pop().unwrap();
                res.push(index as i32);
                time += duration;
            }

            while task < tasks.len() && tasks[task].0 <= time {
                heap.push(Reverse((tasks[task].1, tasks[task].2)));
                task += 1;
            }
        }
        res
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
        "{:?}",
        Solution::get_order(grid![[1, 2], [2, 4], [3, 2], [4, 1]])
    );
    println!(
        "{:?}",
        Solution::get_order(grid![
            [19, 13],
            [16, 9],
            [21, 10],
            [32, 25],
            [37, 4],
            [49, 24],
            [2, 15],
            [38, 41],
            [37, 34],
            [33, 6],
            [45, 4],
            [18, 18],
            [46, 39],
            [12, 24]
        ])
    );
}

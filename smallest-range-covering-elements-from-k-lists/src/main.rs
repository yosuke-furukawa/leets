use std::collections::BinaryHeap;

#[derive(Debug, Clone, Ord, Eq)]
struct Point {
    val: i32,
    group: i32,
    idx: i32,
}

impl std::cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.val.eq(&other.val)
    }
}

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut heap = BinaryHeap::<Point>::new();
        let mut max = std::i32::MIN;
        for g in 0..n {
            let val = nums[g][0];
            if val > max {
                max = val;
            }
            heap.push(Point {
                val,
                group: g as i32,
                idx: 0,
            })
        }
        let mut start = -1;
        let mut end = -1;
        let mut range = std::i32::MAX;

        while heap.len() == n {
            let mut curr = heap.pop().unwrap();
            if max - curr.val < range {
                range = max - curr.val;
                start = curr.val;
                end = max;
            }
            if curr.idx + 1 < nums[curr.group as usize].len() as i32 {
                curr.idx += 1;
                curr.val = nums[curr.group as usize][curr.idx as usize];
                if curr.val > max {
                    max = curr.val;
                }
                heap.push(curr);
            }
        }
        vec![start as i32, end as i32]
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
        Solution::smallest_range(grid![[4, 10, 15, 24, 26], [0, 9, 12, 20], [5, 18, 22, 30]])
    );
}

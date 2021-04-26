use std::collections::BinaryHeap;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut b_left = bricks;
        let mut l_left = ladders;
        let mut furthest: i32 = 0;
        let mut brick_heap = BinaryHeap::new();
        for i in 1..heights.len() {
            if heights[i as usize] > heights[(i - 1) as usize] {
                let needed = heights[i as usize] - heights[(i - 1) as usize];
                if b_left >= needed {
                    brick_heap.push(needed);
                    b_left -= needed;
                } else if l_left > 0 {
                    if let Some(b) = brick_heap.peek() {
                        if *b > needed {
                            b_left += brick_heap.pop().unwrap();
                            b_left -= needed;
                            brick_heap.push(needed);
                        }
                    }
                    l_left -= 1;
                } else {
                    break;
                }
            }
            furthest = i as i32;
        }
        furthest
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1)
    );
}

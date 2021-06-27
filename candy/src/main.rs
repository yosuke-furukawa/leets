use std::collections::VecDeque;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.len() == 1 {
            return 1;
        }
        let mut candies = vec![1; ratings.len()];
        let mut ratings_order: Vec<(usize, i32)> =
            ratings.clone().into_iter().enumerate().collect();
        ratings_order.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        let mut rqueue: VecDeque<(usize, i32)> = ratings_order.into_iter().collect();

        while !rqueue.is_empty() {
            let rating = rqueue.pop_front().unwrap();
            let prevs = if rating.0 > 0 {
                (ratings[rating.0 - 1], candies[rating.0 - 1])
            } else {
                (1_000_000_000, 0)
            };
            let nexts = if rating.0 < ratings.len() - 1 {
                (ratings[rating.0 + 1], candies[rating.0 + 1])
            } else {
                (1_000_000_000, 0)
            };
            if rating.1 > prevs.0 || rating.1 > nexts.0 {
                if rating.1 > prevs.0 && rating.1 > nexts.0 {
                    candies[rating.0] = prevs.1.max(nexts.1) + 1;
                } else if prevs.0 < nexts.0 {
                    candies[rating.0] = prevs.1 + 1;
                } else if prevs.0 > nexts.0 {
                    candies[rating.0] = nexts.1 + 1;
                }
            }
        }

        candies.into_iter().sum()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::candy(vec![1, 0, 2]));
}

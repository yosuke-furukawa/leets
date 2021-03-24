use std::collections::VecDeque;

impl Solution {
    pub fn advantage_count(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut result = vec![-1; a.len()];
        let mut a1: Vec<(i32, bool)> = a.iter().map(|c| (*c, false)).collect();
        let mut b: Vec<(usize, i32)> = b.iter().enumerate().map(|(i, c)| (i, *c)).collect();
        a1.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        b.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        let mut a2: VecDeque<(i32, bool)> = a1.into_iter().collect();

        let mut index = 0;
        while !a2.is_empty() {
            let mut item = a2.pop_front().unwrap();
            if b[index].1 < item.0 {
                result[b[index].0] = item.0;
                index += 1;
            } else if item.1 {
                a2.push_front(item);
                break;
            } else {
                item.1 = true;
                a2.push_back(item);
            }
        }

        for c in result.iter_mut() {
            if *c == -1 {
                *c = a2.pop_front().unwrap().0;
            }
        }

        result
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11])
    );
    println!(
        "{:?}",
        Solution::advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11])
    );
}

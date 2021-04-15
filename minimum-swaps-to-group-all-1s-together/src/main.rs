use std::collections::VecDeque;

impl Solution {
    pub fn min_swaps(data: Vec<i32>) -> i32 {
        let mut min = data.len();
        let ones = data.clone().into_iter().filter(|x| *x > 0).count();
        if ones <= 1 {
            return 0;
        }
        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut zero_count = 0;
        for n in data.iter() {
            if n == &0 {
                zero_count += 1;
            }
            if queue.len() < ones {
                queue.push_back(*n);
                min = zero_count;
                continue;
            }
            let f = queue.pop_front();
            if f == Some(0) {
                zero_count -= 1;
            }
            queue.push_back(*n);
            min = min.min(zero_count);
        }
        min as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_swaps(vec![1,0,1,0,1]));
    println!("{}", Solution::min_swaps(vec![0,0,0,1,0]));
    println!("{}", Solution::min_swaps(vec![1,0,1,0,1,0,0,1,1,0,1]));
    println!("{}", Solution::min_swaps(vec![1,0,1,0,1,0,1,1,1,0,1,0,0,1,1,1,0,0,1,1,1,0,1,0,1,1,0,0,0,1,1,1,1,0,0,1]));
    println!("{}", Solution::min_swaps(vec![1,1,1,1,0,1,0,1,1,1,1,0,1,0,0,1,1,0,0,0,1,0,0,1,1,1,0,0,1,0,0,0,1,1,1,1,1,1,0,1,0,0,1,1,1,1,1,0,1,0,1,0,1,0,1,0,1,1,1,0,1,1,1,0,0,0,0,0,1,1,1,0,0,1,0,1,0,1,1,1,0,1,1,1,0,0,0,1,1,0,1,1,1,0,1,0,1,0,0,1,0,1,1,0,0,1,0,0,1,0,1,1,0,0,1,0,0,0,0,0,1,1,1,1,0,0,1,0,0,0,0,0,0,1,0,0,1,0,1,0,1,1,1,0,0,0,1,1,1,1,1,0,1,0,0,1,1,1,1,1,1,1,1,1,0,1,1,1,0,0,1,0,1,1,1,0,0,0,0,1,0,0,1,0,0,0,0,0,1,1,0,1,0,0,1,0,1,0,1,0]));

}

use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut sum = 0i64;
        let mut heap = BinaryHeap::<i32>::new();
        for x in target {
            heap.push(x);
            sum += x as i64;
        }
        while let Some(mut x) = heap.pop() {
            sum -= x as i64;
            if x == 1 || sum == 1 {
                return true;
            }
            if (x as i64) < sum || sum == 0 || (x as i64) % sum == 0 {
                return false;
            }
            x = ((x as i64) % sum) as i32;
            sum += x as i64;
            heap.push(x);
        }
        false
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::is_possible(vec![9, 3, 5]));
    println!("{}", Solution::is_possible(vec![1, 1, 1, 2]));
    println!("{}", Solution::is_possible(vec![8, 5]));
    println!("{}", Solution::is_possible(vec![7, 5]));
    println!("{}", Solution::is_possible(vec![5, 2]));
    println!("{}", Solution::is_possible(vec![1, 1000000000]));
    println!("{}", Solution::is_possible(vec![9, 9, 9]));
    println!("{}", Solution::is_possible(vec![2, 900000001]));
    println!("{}", Solution::is_possible(vec![2]));
}

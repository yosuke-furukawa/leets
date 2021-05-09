use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut arrays = BinaryHeap::new();
        let mut sum = 0;
        for n in target {
            sum += n;
            arrays.push(n);
        }
        while let Some(num) = arrays.pop() {
            if num == 1 {
                break;
            }
            sum -= num;
            if sum < 1 || num <= sum {
                return false;
            }
            let n = num % sum;
            arrays.push(n);
            sum += n;
        }
        true
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

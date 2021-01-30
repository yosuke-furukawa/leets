use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        for n in nums.iter() {
            if n % 2 == 0 {
                heap.push(*n);
            } else {
                heap.push(n * 2);
            }
        }
        let mut min = *heap.iter().min().unwrap();
        let mut diff = 1000000000;
        while let Some(max) = heap.pop() {
            diff = diff.min(max - min);
            if max % 2 == 0 {
                min = min.min(max / 2);
                heap.push(max / 2);
            } else {
                break;
            }
        }
        diff
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::minimum_deviation(vec![1,2,3,4]));
    println!("{}", Solution::minimum_deviation(vec![4,1,5,20,3]));
    println!("{}", Solution::minimum_deviation(vec![2,10,8]));
    println!("{}", Solution::minimum_deviation(vec![3,5]));
    println!("{}", Solution::minimum_deviation(vec![2,8,6,1,6]));
}

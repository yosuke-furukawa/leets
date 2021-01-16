use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut heap = BinaryHeap::from(nums);

        let mut n = Some(0 as i32);
        while k > 0 {
            n = heap.pop();
            k -= 1;
        }

        n.unwrap()
    }
}
struct Solution;

fn main() {
    println!("{}", Solution::find_kth_largest(vec![3,2,1,5,6,4], 2));
    println!("{}", Solution::find_kth_largest(vec![3,2,3,1,2,4,5,5,6], 4));
}

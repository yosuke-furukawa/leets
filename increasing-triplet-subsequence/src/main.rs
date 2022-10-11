impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut lo = i32::MAX;
        let mut mid = i32::MAX;
        for n in nums {
            if n <= lo {
                lo = n;
            } else if n <= mid {
                mid = n;
            } else {
                return true;
            }
        }
        false
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::increasing_triplet(vec![1, 2, 3, 4, 5]));
}

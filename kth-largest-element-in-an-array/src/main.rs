impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        nums[(k - 1) as usize]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_kth_largest(vec![3,2,1,5,6,4], 2));
    println!("{}", Solution::find_kth_largest(vec![3,2,3,1,2,4,5,5,6], 4));
}

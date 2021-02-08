impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        for i in 1..nums.len() {
            if (i % 2 == 0) == (nums[i] > nums[i - 1]) {
                nums.swap(i, i - 1);
            }
        }
    }
}

struct Solution;

fn main() {
    let mut v = vec![3, 5, 2, 1, 6, 4];
    Solution::wiggle_sort(&mut v);
    println!("{:?}", v);
}

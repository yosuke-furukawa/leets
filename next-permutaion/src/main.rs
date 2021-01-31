impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut start = nums.len() - 1;
        while start > 0 && nums[start - 1] >= nums[start] {
            start -= 1;
        }

        if start > 0 {
            let mut end = nums.len() - 1;
            while end >= start && nums[end] <= nums[start - 1] {
                end -= 1;
            }
            nums.swap(start - 1, end);
        }
        nums[start..].reverse();
    }
}

struct Solution;

fn main() {
    let mut v = vec![1, 2, 3];
    Solution::next_permutation(&mut v);
    println!("{:?}", v);
    let mut v = vec![3, 2, 1];
    Solution::next_permutation(&mut v);
    println!("{:?}", v);
    let mut v = vec![1, 1, 5];
    Solution::next_permutation(&mut v);
    println!("{:?}", v);
    let mut v = vec![2, 1, 3];
    Solution::next_permutation(&mut v);
    println!("{:?}", v);
}

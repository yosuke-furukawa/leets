impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut evens = vec![];
        let mut odds = vec![];

        for n in nums.iter() {
            if n % 2 == 0 {
                evens.push(*n);
            } else {
                odds.push(*n);
            }
        }
        for (i, n) in nums.iter_mut().enumerate() {
            if i % 2 == 0 {
                *n = evens.pop().unwrap();
            } else {
                *n = odds.pop().unwrap();
            }
        }
        nums
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]));
}

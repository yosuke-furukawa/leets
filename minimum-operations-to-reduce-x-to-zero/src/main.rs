impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut current = nums.iter().sum::<i32>();
        let mut min = std::i32::MAX;
        let mut l = 0;
        for r in 0..nums.len() as i32 {
            current -= nums[r as usize];
            while current < x && l <= r {
                current += nums[l as usize];
                l += 1;
            }
            if current == x {
                min = min.min(nums.len() as i32 - 1 - r + l);
            }
        }
        if min == std::i32::MAX {
            -1
        } else {
            min
        }
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_operations(vec![1, 1, 4, 2, 3], 5));
    println!("{}", Solution::min_operations(vec![5, 6, 7, 8, 9], 4));
    println!("{}", Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10));
}

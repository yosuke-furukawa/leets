    impl Solution {
        pub fn jump(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            if n < 2 {
                return 0;
            }

            let mut max_pos = nums[0];
            let mut max_steps = nums[0];

            let mut jumps = 1;
            for (i, n) in nums.iter().enumerate().take(n).skip(1) {
                if max_steps < i as i32 {
                    jumps += 1;
                    max_steps = max_pos;
                }
                max_pos = max_pos.max(n + i as i32);
            }
            jumps
        }
    }

struct Solution;

fn main() {
    println!("{}", Solution::jump(vec![2, 3, 1, 1, 4]));
    println!("{}", Solution::jump(vec![2, 3, 0, 1, 4]));
}

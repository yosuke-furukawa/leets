impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut answer = 0;
        while left < right {
            if height[left] < height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    answer += left_max - height[left];
                }
                left += 1;
            } else {
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    answer += right_max - height[right];
                }
                right -= 1;
            }
        }
        answer
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])
    );
}

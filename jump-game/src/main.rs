impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let goal = nums.len() as i32 - 1;
        if goal == 0 {
            return true;
        }
        let mut pos = goal;
        for i in (0..goal).rev() {
            let to = i + nums[i as usize];
            if to >= goal || to >= pos {
                pos = i;
            }
        }
        pos == 0
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::can_jump(vec![2, 3, 1, 1, 4]));
}

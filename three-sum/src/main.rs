impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();

        if nums.is_empty() {
            return ret;
        }
        if nums.len() == 1 {
            return ret;
        }

        let mut nums = nums;
        nums.sort_unstable();

        for i in 0..(nums.len() - 2) {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            if nums[i] > 0 {
                return ret;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let l = nums[left];
                let r = nums[right];
                let x = nums[i];
                let total = l + r + x;
                if total > 0 {
                    right -= 1;
                } else if total < 0 {
                    left += 1;
                } else {
                    ret.push(vec![l, r, x]);
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }

                    left += 1;
                    right -= 1;
                }
            }
        }
        ret
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]));
}

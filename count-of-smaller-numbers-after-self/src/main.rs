use std::collections::VecDeque;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        queue.push_back(0);
        let mut temp = vec![nums[nums.len() - 1]];
        for n in nums.into_iter().rev().skip(1) {
            let mut left = 0;
            let mut right = temp.len();
            let mut mid = (left + right) / 2;
            while left <= right {
                if mid < temp.len() && n < temp[mid] {
                    if mid == 0 {
                        break;
                    }
                    right = mid - 1;
                } else if mid < temp.len() && n > temp[mid] {
                    left = mid + 1;
                } else {
                    if right == 0 {
                        break;
                    }
                    right -= 1;
                }
                mid = (left + right) / 2;
            }
            if left > right {
                temp.insert(mid + 1, n);
                queue.push_front(mid as i32 + 1);
            } else {
                temp.insert(mid, n);
                queue.push_front(mid as i32);
            }
        }
        queue.into_iter().collect()
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::count_smaller(vec![5, 2, 6, 1]));
    println!("{:?}", Solution::count_smaller(vec![-1, -1]));
    println!("{:?}", Solution::count_smaller(vec![-1, 0]));
}

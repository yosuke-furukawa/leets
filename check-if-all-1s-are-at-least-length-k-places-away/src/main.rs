use std::cmp::min;

const MAX: i32 = 100000;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        if k == 1 {
            return true;
        }

        let mut min_diff = MAX;
        let mut diff = -1;
        for num in nums {
            match num {
                1 => {
                    if diff >= 0 {
                        min_diff = min(min_diff, diff);
                    }
                    diff = 0;
                },
                0 if diff >= 0 => {
                    diff += 1;
                },
                _ => continue,
            }
        }
        if min_diff == MAX {
            return true;
        }
        min_diff == k
    }
}

struct Solution;


fn main() {
    println!("{}", Solution::k_length_apart(vec![1,0,0,0,1,0,0,1], 2));
    println!("{}", Solution::k_length_apart(vec![1,0,0,1,0,1], 2));
    println!("{}", Solution::k_length_apart(vec![1,1,1,1,1], 0));
    println!("{}", Solution::k_length_apart(vec![0,1,0,1], 1));   
}

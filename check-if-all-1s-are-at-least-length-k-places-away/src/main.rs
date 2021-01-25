use std::cmp::min;

const MAX: i32 = 100000;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        if k == 1 {
            return true;
        }

        let result = nums.iter().try_fold((MAX, -1), |(min_diff, diff), num| {
            match num {
                1 => {
                    let mut m = min_diff;
                    if diff >= 0 {
                        m = min(min_diff, diff);
                        if m < k {
                            return None;
                        }
                    }
                    Some((m, 0))
                },
                0 if diff >= 0 => {
                    Some((min_diff, diff + 1))
                },
                _ => Some((min_diff, diff)),
            }
        });


        if result == None {
            return false;
        }
        let (min_diff, _) = result.unwrap();
        min_diff == k || min_diff == MAX
    }
}

struct Solution;


fn main() {
    println!("{}", Solution::k_length_apart(vec![1,0,0,0,1,0,0,1], 2));
    println!("{}", Solution::k_length_apart(vec![1,0,0,1,0,1], 2));
    println!("{}", Solution::k_length_apart(vec![1,1,1,1,1], 0));
    println!("{}", Solution::k_length_apart(vec![0,1,0,1], 1));   
}

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0;
        } else if nums.len() == 2 {
            return (nums[1] - nums[0]).abs();
        }
        let n = nums.len();
        use std::i32::{MAX, MIN};
        let mut maxs = vec![MIN; n];
        let mut mins = vec![MAX; n];
        let max = *nums.iter().max().unwrap();
        let min = *nums.iter().min().unwrap();
        if max == min {
            return 0;
        }

        let avggap = (max - min) as f64 / (n as f64 - 1.0);
        for &v in nums.iter() {
            let idx = ((v - min) as f64 / avggap) as usize;
            mins[idx] = mins[idx].min(v);
            maxs[idx] = maxs[idx].max(v);
        }

        let premax = maxs[0];
        mins.into_iter()
            .zip(maxs)
            .skip(1)
            .scan(premax, |premax, (cur_min, cur_max)| {
                if cur_min <= cur_max {
                    let premax_ = *premax;
                    *premax = cur_max;
                    Some(cur_min - premax_)
                } else {
                    Some(MIN)
                }
            })
            .max()
            .unwrap()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::maximum_gap(vec![3, 6, 9, 1]));
    println!("{}", Solution::maximum_gap(vec![10]));
}

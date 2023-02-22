impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut left = weights.clone().into_iter().max().unwrap();
        let mut right = weights.clone().into_iter().sum::<i32>();
        while left < right {
            let mid = left + (right - left) / 2;
            let mut count = 1;
            let mut sum = 0;
            for &w in &weights {
                if sum + w > mid {
                    count += 1;
                    sum = w;
                } else {
                    sum += w;
                }
            }
            if count > days {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5)
    );
}

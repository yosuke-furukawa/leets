impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let mut sums: Vec<i128> = vec![];
        for n in nums.iter() {
            sums.push(sums.last().unwrap_or(&0) + *n as i128);
        }
        let n = nums.len();
        let mut min = std::i128::MAX;
        let mut res = -1;
        for (i, sum) in sums.iter().enumerate() {
            if n - i - 1 == 0 {
                if min > sum / n as i128 {
                    res = i as i32;
                }
                break;
            }
            let diff = (sum / (i + 1) as i128 - (sums[n - 1] - sum) / (n - i - 1) as i128).abs();
            if min > diff {
                min = diff;
                res = i as i32;
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::minimum_average_difference(vec![2, 5, 3, 9, 5, 3])
    );
}

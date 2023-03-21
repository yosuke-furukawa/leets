impl Solution {
    fn sum(zeros: i64) -> i64 {
        (1 + zeros) * zeros / 2
    }
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        let mut zeros = 0;
        for n in nums {
            if n == 0 {
                zeros += 1;
                continue;
            }
            res += Self::sum(zeros as i64);
            zeros = 0;
        }
        if zeros != 0 {
            res += Self::sum(zeros as i64);
        }
        res
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4])
    );
}

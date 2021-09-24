impl Solution {
    fn kadane(nums: &[i32], i: usize, j: usize, sign: i32) -> i32 {
        let mut ans = std::i32::MIN;
        let mut cur = std::i32::MIN;
        for num in nums.iter().take(j + 1).skip(i) {
            cur = sign * num + cur.max(0);
            ans = ans.max(cur);
        }
        ans
    }
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let len = nums.len();
        for x in nums.iter() {
            sum += x;
        }
        let ans1 = Self::kadane(&nums, 0, len - 1, 1);
        let ans2 = if len > 1 {
            sum + Self::kadane(&nums, 1, len - 1, -1)
        } else {
            ans1
        };
        let ans3 = if len > 2 {
            sum + Self::kadane(&nums, 0, len - 2, -1)
        } else {
            ans2
        };
        ans1.max(ans2.max(ans3))
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::max_subarray_sum_circular(vec![1, -2, 3, -2])
    );
    println!("{}", Solution::max_subarray_sum_circular(vec![-2]));
}

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut stack: Vec<i64> = vec![];
        let mut sum: i64 = 0;
        for i in 0..=n {
            while !stack.is_empty()
                && (i == arr.len() || arr[*stack.last().unwrap() as usize] > arr[i])
            {
                let mid = stack.pop().unwrap();
                let left = if stack.is_empty() {
                    -1
                } else {
                    *stack.last().unwrap()
                };
                let right = i as i64;
                let count = (mid - left) * (right - mid);
                sum += (count * arr[mid as usize] as i64) % 1000000007;
                sum %= 1000000007;
            }
            stack.push(i as i64);
        }
        sum as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::sum_subarray_mins(vec![3, 1, 2, 4]));
}

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![-1; n];
        let mut stack = Vec::<usize>::new();

        for _ in 0..2 {
            for i in (0..n).rev() {
                let elem = nums[i];
                let mut greater = -1;
                while let Some(&v) = stack.last() {
                    if nums[v] > elem {
                        greater = nums[v];
                        break;
                    }
                    stack.pop();
                }
                result[i] = greater;
                stack.push(i);
            }
        }

        result
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::next_greater_elements(vec![1, 2, 1]));
    println!("{:?}", Solution::next_greater_elements(vec![1, 2, 3, 4, 3]));
}

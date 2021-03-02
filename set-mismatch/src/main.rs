impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut array = vec![false; nums.len()];
        let mut result = vec![];
        for n in nums {
            match array[(n - 1) as usize] {
                false => array[(n - 1) as usize] = true,
                true => result.push(n),
            }
        }
        for (i, v) in array.iter().enumerate() {
            if !v {
                result.push((i + 1) as i32);
                break;
            }
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::find_error_nums(vec![1, 2, 2, 4]));
}

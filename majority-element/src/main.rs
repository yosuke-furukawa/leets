impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;
        for num in nums {
            if count == 0 {
                candidate = num;
                count += 1;
            } else if candidate == num {
                count += 1;
            } else {
                count -= 1;
            }
        }
        candidate
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::majority_element(vec![3, 2, 3, 2, 2, 3, 3]));
}

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        let mut s = String::new();
        if nums.len() == 1 {
            return nums[0].to_string();
        }
        if nums.len() == 2 {
            return format!("{}/{}", nums[0], nums[1]);
        }
        for i in 0..nums.len() {
            s += &(nums[i].to_string() + "/");
            if i == 0 {
                s += "(";
            }
        }
        s.pop();
        s.push(')');
        s
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::optimal_division(vec![1000, 100, 10, 2]));
}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            let mask = 1 << i;
            let mut count = 0;
            for num in nums.iter() {
                if num & mask != 0 {
                    count += 1;
                }
            }
            if count % 3 != 0 {
                res |= mask;
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::single_number(vec![2, 2, 3, 2]));
    println!("{}", Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]));
}

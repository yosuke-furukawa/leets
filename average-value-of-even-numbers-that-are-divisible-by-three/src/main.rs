impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut count = 0;
        for n in nums {
            if n % 3 == 0 && n % 2 == 0 {
                sum += n;
                count += 1;
            }
        }
        if count == 0 {
            return 0;
        }
        sum / count
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::average_value(vec![1, 3, 6, 10, 12, 15]));
}

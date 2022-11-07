impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut nums = vec![];
        let mut n = num;
        while n > 0 {
            nums.push(n % 10);
            n /= 10;
        }
        let mut res = 0;
        let mut found = false;
        while let Some(n) = nums.pop() {
            if !found && n == 6 {
                res = res * 10 + 9;
                found = true;
                continue;
            }
            res = res * 10 + n;
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::maximum69_number(9669));
}

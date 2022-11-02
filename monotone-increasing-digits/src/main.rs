impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut digits = vec![];
        let mut n = n;
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();
        let mut begin = -1;
        let mut same = true;
        for i in 1..digits.len() {
            if begin == -1 && digits[i - 1] >= digits[i] {
                begin = i as i32 - 1;
            }
            if digits[i - 1] > digits[i] {
                same = false;
                break;
            }
            if digits[i - 1] < digits[i] {
                begin = i as i32;
            }
        }
        if begin >= 0 && begin < digits.len() as i32 - 1 && !same {
            digits[begin as usize] -= 1;
            for d in digits.iter_mut().skip((begin + 1) as usize) {
                *d = 9;
            }
        }
        let mut result = 0;
        for d in digits {
            result = result * 10 + d;
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::monotone_increasing_digits(10));
    println!("{}", Solution::monotone_increasing_digits(1234));
    println!("{}", Solution::monotone_increasing_digits(332));
    println!("{}", Solution::monotone_increasing_digits(668841));
    println!("{}", Solution::monotone_increasing_digits(101));
    println!("{}", Solution::monotone_increasing_digits(11));
    println!("{}", Solution::monotone_increasing_digits(0));
    println!("{}", Solution::monotone_increasing_digits(1));
}

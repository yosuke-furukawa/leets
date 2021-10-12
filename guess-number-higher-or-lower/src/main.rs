impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut l: i64 = 0;
        let mut r: i64 = n as i64;
        let mut num = (l + r) / 2;
        while l < r {
            let e = guess(num as i32);
            match e {
                1 => l = num + 1,
                -1 => r = num - 1,
                _ => break,
            }
            num = (l + r) / 2;
        }
        num as i32
    }
}

struct Solution;

unsafe fn guess(num: i32) -> i32 {
    let temp_guess_num = 1702766719;
    (temp_guess_num - num).signum()
}

fn main() {
    unsafe {
        println!("{}", Solution::guessNumber(100));
        println!("{}", Solution::guessNumber(2126753390));
    }
}

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for _ in 1..n {
            let mut new_vec = vec![];
            for num in vec {
                if num == 0 {
                    continue;
                }
                let last_digit = num % 10;
                if last_digit + k < 10 {
                    new_vec.push(num * 10 + last_digit + k);
                }
                if k != 0 && last_digit >= k {
                    new_vec.push(num * 10 + last_digit - k);
                }
            }
            vec = new_vec;
        }
        vec
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::nums_same_consec_diff(3, 7));
    println!("{:?}", Solution::nums_same_consec_diff(2, 1));
}

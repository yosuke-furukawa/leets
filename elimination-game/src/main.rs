impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut is_left = true;
        let mut remain = n;
        let mut result = 1;
        let mut jump = 1;

        while remain > 1 {
            if is_left || remain % 2 != 0 {
                result += jump;
            }

            remain /= 2;
            jump *= 2;
            is_left = !is_left;
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::last_remaining(9));
}

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut i = 0;
        let mut count = 0;
        while i < data.len() {
            let d = data[i];

            if count == 0 {
                if d >> 5 == 0b110 {
                    count = 1;
                } else if d >> 4 == 0b1110 {
                    count = 2;
                } else if d >> 3 == 0b11110 {
                    count = 3;
                } else if d >> 7 == 0b0 {
                    // noop
                } else {
                    return false;
                }
            } else {
                if d >> 6 != 0b10 {
                    return false;
                }
                count -= 1;
            }
            i += 1;
        }
        count == 0
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::valid_utf8(vec![197, 130, 1]));
    println!("{}", Solution::valid_utf8(vec![235, 140, 4]));
}

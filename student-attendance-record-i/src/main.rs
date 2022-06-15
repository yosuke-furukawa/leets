impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut count = 0;
        if s.chars().filter(|x| *x == 'A').count() > 1 {
            return false;
        }
        for i in s.chars() {
            if i == 'L' {
                count += 1;
                if count >= 3 {
                    return false;
                }
            } else {
                count = 0;
            }
        }
        true
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::check_record("PPALLP".to_string()));
    println!("{}", Solution::check_record("PPALLL".to_string()));
}

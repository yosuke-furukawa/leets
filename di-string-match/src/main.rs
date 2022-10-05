impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let n = s.len();
        let mut res = vec![0; n + 1];
        let mut low = 0;
        let mut high = n as i32;
        for (i, c) in s.chars().enumerate() {
            match c {
                'I' => {
                    res[i] = low;
                    low += 1;
                }
                'D' => {
                    res[i] = high;
                    high -= 1;
                }
                _ => unreachable!(),
            }
        }
        res[n] = low;
        res
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::di_string_match("IDID".to_string()));
}

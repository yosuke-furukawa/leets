impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut dp = vec![0];
        let mut count = 0;
        for s in arr {
            let mut a = 0;
            let mut dup = 0;
            for c in s.chars() {
                dup |= a & 1 << (c as i32 - 'a' as i32);
                a |= 1 << (c as i32 - 'a' as i32)
            }
            if dup > 0 {
                continue;
            }
            for n in dp.clone().iter().rev() {
                if n & a > 0 {
                    continue;
                }
                dp.push(n | a);
                count = count.max(((n | a) as i32).count_ones());
            }
        }
        count as i32
    }
}

struct Solution;

fn stringify(s: Vec<&str>) -> Vec<String> {
    s.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::max_length(stringify(vec!["un", "iq", "ue"]))
    );
}

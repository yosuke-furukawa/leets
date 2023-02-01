impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let (mut s1, mut s2) = if str1.len() < str2.len() {
            (str2, str1)
        } else {
            (str1, str2)
        };
        let mut gcd = "".to_string();
        while s1.starts_with(&s2) {
            if s2.is_empty() {
                break;
            }
            gcd = s2.clone();
            s1 = s1.replace(&s2, "");
            if s1.len() < s2.len() {
                std::mem::swap(&mut s1, &mut s2);
            }
        }
        if s2.is_empty() {
            gcd
        } else {
            "".to_string()
        }
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string())
    );
    println!(
        "{}",
        Solution::gcd_of_strings("ABABAB".to_string(), "AB".to_string())
    );
    println!(
        "{}",
        Solution::gcd_of_strings("EFGABC".to_string(), "ABC".to_string())
    );
}

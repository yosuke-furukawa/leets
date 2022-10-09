impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let mut s = a.clone();
        let mut count = 1;
        while s.len() < b.len() {
            s.push_str(&a);
            count += 1;
        }
        if s.contains(&b) {
            return count;
        }
        s.push_str(&a);
        count += 1;
        if s.contains(&b) {
            return count;
        }
        -1
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::repeated_string_match("abcd".to_string(), "cdabcdab".to_string()));
}

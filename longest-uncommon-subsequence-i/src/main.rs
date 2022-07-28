impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            -1
        } else {
            a.len().max(b.len()) as i32
        }
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::find_lu_slength("aba".to_string(), "cdc".to_string())
    );
}

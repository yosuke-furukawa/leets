impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut res = 0;
        for i in 0..strs[0].len() {
            for j in 1..strs.len() {
                if strs[j].as_bytes()[i] < strs[j - 1].as_bytes()[i] {
                    res += 1;
                    break;
                }
            }
        }
        res
    }
}

struct Solution;

fn stringify(s: Vec<&str>) -> Vec<String> {
    s.into_iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::min_deletion_size(stringify(vec!["cba", "daf", "ghi"]))
    );
}

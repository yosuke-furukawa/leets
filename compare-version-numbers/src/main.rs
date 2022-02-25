impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1s: Vec<&str> = version1.split('.').collect();
        let v2s: Vec<&str> = version2.split('.').collect();
        let len = v1s.len().max(v2s.len());
        for i in 0..len {
            let v1 = v1s.get(i).unwrap_or(&"0").parse::<i32>().unwrap_or(0);
            let v2 = v2s.get(i).unwrap_or(&"0").parse::<i32>().unwrap_or(0);
            if v1 == v2 {
                continue;
            }

            if v1 < v2 {
                return -1;
            } else {
                return 1;
            }
        }
        0
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::compare_version("1.01".to_string(), "1.001".to_string())
    );
}

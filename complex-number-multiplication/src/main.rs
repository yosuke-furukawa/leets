impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let nums1: Vec<&str> = num1.split('+').collect();
        let nums2: Vec<&str> = num2.split('+').collect();
        let r1 = nums1[0].parse::<i32>().unwrap();
        let i1 = nums1[1].strip_suffix('i').unwrap().parse::<i32>().unwrap();
        let r2 = nums2[0].parse::<i32>().unwrap();
        let i2 = nums2[1].strip_suffix('i').unwrap().parse::<i32>().unwrap();
        let r3 = r1 * r2 - i1 * i2;
        let i3 = r1 * i2 + r2 * i1;
        format!("{}+{}i", r3, i3)
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::complex_number_multiply("1+1i".to_string(), "1+1i".to_string())
    );
    println!(
        "{}",
        Solution::complex_number_multiply("1+-1i".to_string(), "1+-1i".to_string())
    );
    println!(
        "{}",
        Solution::complex_number_multiply("1+1i".to_string(), "1+-1i".to_string())
    );
}

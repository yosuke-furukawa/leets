impl Solution {
    pub fn is_number(s: String) -> bool {
        if s.contains("inf") {
            return false;
        }
        s.parse::<f64>().is_ok()
    }
}

struct Solution;

fn main() {
    println!("valid case");

    for valid in &[
        "2",
        "0089",
        "-0.1",
        "+3.14",
        "4.",
        "-.9",
        "2e10",
        "-90E3",
        "3e+7",
        "+6e-1",
        "53.5e93",
        "-123.456e789",
    ] {
        println!("{}", Solution::is_number(valid.to_string()));
    }

    println!("invalid case");
    for invalid in &[
        "abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53", "Infinity", "inf",
    ] {
        println!("{}", Solution::is_number(invalid.to_string()));
    }
}

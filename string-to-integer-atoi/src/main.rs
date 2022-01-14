impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut value: i64 = 0;
        let max = std::i32::MAX as i64;
        let min = std::i32::MIN as i64;
        let chars: Vec<char> = s.trim().chars().collect();
        let mut skipped = 0;
        let sign = match chars.get(0) {
            Some('-') => {
                skipped = 1;
                -1
            }
            Some('+') => {
                skipped = 1;
                1
            }
            _ => 1,
        };
        for c in chars.iter().skip(skipped) {
            if let Some(n) = c.to_digit(10) {
                value = value * 10 + n as i64;
                if value > max {
                    break;
                }
            } else {
                break;
            }
        }
        let ans = value * sign;
        if max < ans {
            max as i32
        } else if min > ans {
            min as i32
        } else {
            ans as i32
        }
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::my_atoi("42".to_string()));
    println!("{}", Solution::my_atoi("       -42".to_string()));
    println!("{}", Solution::my_atoi("4193 with words".to_string()));
    println!("{}", Solution::my_atoi("".to_string()));
    println!("{}", Solution::my_atoi("+42".to_string()));
    println!("{}", Solution::my_atoi("9223372036854775808".to_string()));
    println!(
        "{}",
        Solution::my_atoi("1234567890123456789012345678901234567890".to_string())
    );
}

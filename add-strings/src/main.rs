impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let len = if num1.len() < num2.len() { num1.len() } else { num2.len() };
        let revnum1: Vec<i32> = num1.chars().map(|c| c as i32 - '0' as i32).rev().collect();
        let revnum2: Vec<i32> = num2.chars().map(|c| c as i32 - '0' as i32).rev().collect();
        let mut results: Vec<i32> = vec![];
        let mut carry_on = 0;
        for i in 0..len {
            let sum = revnum1[i] + revnum2[i] + carry_on;
            let ans = sum % 10;
            carry_on = sum / 10;
            results.push(ans);
        }
        let rests = if num1.len() < num2.len() { revnum2 } else { revnum1 };
        for i in len..rests.len() {
            let sum = rests[i] + carry_on;
            let ans = sum % 10;
            carry_on = sum / 10;
            results.push(ans);
        }
        if carry_on > 0 {
            results.push(carry_on);
        }
        results.iter().rev().fold(String::new(), |acc, x| acc + &x.to_string())
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::add_strings("112".to_string(), "23".to_string()));
    println!("{}", Solution::add_strings("456".to_string(), "77".to_string()));
    println!("{}", Solution::add_strings("1".to_string(), "9".to_string()));
}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        let char1: Vec<char> = num1.chars().rev().collect();
        let char2: Vec<char> = num2.chars().rev().collect();
        let mut ans = vec![0; num1.len() + num2.len()];

        for (i2, c2) in char2.iter().enumerate() {
            let d2 = *c2 as i32 - '0' as i32;
            for (i1, c1) in char1.iter().enumerate() {
                let d1 = *c1 as i32 - '0' as i32;
                let pos = i1 + i2;
                let carry = ans[pos];
                let multiply = d1 * d2 + carry;
                ans[pos] = multiply % 10;
                let val = ans[pos + 1] + multiply / 10;
                ans[pos + 1] = val;
            }
        }

        if ans[ans.len() - 1] == 0 {
            ans.pop();
        }
        ans.iter().rev().map(|x| x.to_string()).collect()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::multiply("2".to_string(), "3".to_string()));
    println!(
        "{}",
        Solution::multiply("123".to_string(), "456".to_string())
    );
}

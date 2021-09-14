impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut result = vec!['\0'; s.len()];
        let mut l = 0;
        let mut r = s.len() - 1;
        while l <= r {
            let c1 = chars[l];
            let c2 = chars[r];
            if c1.is_ascii_alphabetic() && c2.is_ascii_alphabetic() {
                result[l] = c2;
                result[r] = c1;
                l += 1;
                if r == 0 {
                    break;
                }
                r -= 1;
            } else {
                if !c1.is_ascii_alphabetic() {
                    result[l] = c1;
                    l += 1;
                }
                if !c2.is_ascii_alphabetic() {
                    result[r] = c2;
                    if r == 0 {
                        break;
                    }
                    r -= 1;
                }
            }
        }
        result
            .into_iter()
            .fold(String::new(), |acc, cur| acc + &cur.to_string())
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::reverse_only_letters("ab-cd".to_string()));
    println!(
        "{}",
        Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string())
    );
    println!(
        "{}",
        Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string())
    );
    println!("{}", Solution::reverse_only_letters("a".to_string()));
}

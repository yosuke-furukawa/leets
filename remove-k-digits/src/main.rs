impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        let mut stack: Vec<char> = vec![];
        for c in num.chars() {
            while let Some(&top) = stack.last() {
                if k > 0 && top > c {
                    stack.pop();
                    k -= 1;
                } else {
                    break;
                }
            }
            stack.push(c);
        }
        while k != 0 {
            stack.pop();
            k -= 1;
        }

        let res: String = stack.into_iter().skip_while(|&c| c == '0').collect();
        if res.is_empty() {
            "0".to_string()
        } else {
            res
        }
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::remove_kdigits("1432219".to_string(), 3));
    println!("{}", Solution::remove_kdigits("10200".to_string(), 1));
    println!("{}", Solution::remove_kdigits("10".to_string(), 2));
}

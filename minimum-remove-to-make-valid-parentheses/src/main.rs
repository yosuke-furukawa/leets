impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack = vec![];
        let mut result = vec![];
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => {
                    stack.push((i, c));
                    result.push(c);
                }
                ')' => {
                    let v = stack.pop();
                    match v {
                        None => result.push(' '),
                        Some(v) => {
                            if v.1 == '(' {
                                result.push(c);
                            }
                        }
                    }
                }
                _ => {
                    result.push(c);
                }
            }
        }
        if !stack.is_empty() {
            for v in stack {
                result[v.0] = ' ';
            }
        }

        result.iter().fold("".to_string(), |acc, s| {
            if s != &' ' {
                acc + s.to_string().as_str()
            } else {
                acc
            }
        })
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string())
    );
    println!(
        "{}",
        Solution::min_remove_to_make_valid("a)b(c)d".to_string())
    );
    println!("{}", Solution::min_remove_to_make_valid("))((".to_string()));
    println!(
        "{}",
        Solution::min_remove_to_make_valid("(a(b(c)d)".to_string())
    );
}

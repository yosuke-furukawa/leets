impl Solution {
    pub fn expand(s: String) -> Vec<String> {
        let mut result = vec![];
        let mut temp: Vec<String> = vec![];
        let mut start_brace = false;
        for c in s.chars() {
            match c {
                ',' => {
                    continue;
                }
                '{' => {
                    start_brace = true;
                }
                '}' => {
                    let mut new_result = vec![];
                    for t in temp.iter().cloned() {
                        if result.len() > 0 {
                            for r in result.iter().cloned() {
                                new_result.push(r + t.as_str());
                            }
                        } else {
                            new_result.push(t);
                        }
                    }
                    result = new_result;
                    temp = vec![];
                    start_brace = false;
                }
                ch if start_brace && ch.is_ascii() => {
                    temp.push(ch.to_string());
                }
                ch => {
                    if result.len() > 0 {
                        for a in result.iter_mut() {
                            *a += ch.to_string().as_str();
                        }
                    } else {
                        result.push(ch.to_string());
                    }
                }
            }
        }
        result.sort_unstable();
        result
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::expand("{a,b}c{d,e}f".to_string()));
}

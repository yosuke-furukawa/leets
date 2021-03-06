impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut encodes: Vec<String> = vec![];
        for ws in words {
            let w = ws.as_str();
            let mut exists = false;
            for encode in encodes.iter_mut() {
                if encode.ends_with(w) || ws.ends_with(encode.as_str()) {
                    exists = true;
                    if encode.len() < ws.len() {
                        *encode = ws.clone()
                    }
                    break;
                }
            }
            if !exists {
                encodes.push(ws);
            }
        }
        encodes.join("#").len() as i32 + 1
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::minimum_length_encoding(vec![
            "time".to_string(),
            "me".to_string(),
            "bell".to_string()
        ])
    );
    println!(
        "{}",
        Solution::minimum_length_encoding(vec!["me".to_string(), "time".to_string()])
    );
}

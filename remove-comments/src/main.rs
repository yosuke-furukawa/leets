impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let mut start_block = false;
        let mut res_line = String::new();
        for line in source {
            let chars = line.chars().collect::<Vec<_>>();
            let mut inline_comment = false;
            let mut skip_next_token = false;
            for i in 1..chars.len() {
                if skip_next_token {
                    skip_next_token = false;
                    continue;
                }
                if start_block {
                    if chars[i - 1] == '*' && chars[i] == '/' {
                        start_block = false;
                        skip_next_token = true;
                    }
                    continue;
                }
                if chars[i - 1] == '/' && chars[i] == '/' {
                    inline_comment = true;
                    break;
                }
                if chars[i - 1] == '/' && chars[i] == '*' {
                    start_block = true;
                    skip_next_token = true;
                    continue;
                }
                res_line.push(chars[i - 1]);
            }
            if !start_block && !inline_comment && !skip_next_token {
                res_line.push(chars[chars.len() - 1]);
            }
            if start_block {
                continue;
            }
            if !res_line.is_empty() {
                res.push(res_line.clone());
                res_line.clear();
            }
        }
        res
    }
}

struct Solution;

fn stringify(s: Vec<&str>) -> Vec<String> {
    s.into_iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{:?}",
        Solution::remove_comments(stringify(vec![
            "/*Test program */",
            "int main()",
            "{ ",
            "  // variable declaration ",
            "int a, b, c;",
            "/* This is a test",
            "   multiline  ",
            "   comment for ",
            "   testing */",
            "a = b + c;",
            "}"
        ]))
    );
    println!(
        "{:?}",
        Solution::remove_comments(stringify(vec!["a/*comment", "line", "more_comment*/b"]))
    );
}

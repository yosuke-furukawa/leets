impl Solution {
    fn tokenize_string(s: &str) -> Vec<String> {
        let mut tokens: Vec<String> = vec![];
        let mut special_tokens = vec![];
        for c in s.chars() {
            if !special_tokens.is_empty() && special_tokens[0] == "&" && c == ';' {
                special_tokens.push(c.to_string());
                tokens.push(match special_tokens.join("").as_str() {
                    "&quot;" => "\"".to_string(),
                    "&apos;" => "'".to_string(),
                    "&amp;" => "&".to_string(),
                    "&gt;" => ">".to_string(),
                    "&lt;" => "<".to_string(),
                    "&frasl;" => "/".to_string(),
                    s => s.to_string(),
                });
                special_tokens.clear();
            } else if c == '&' {
                if special_tokens.is_empty() {
                    special_tokens.push(c.to_string());
                } else {
                    tokens.push(special_tokens.join("").to_string());
                    special_tokens.clear();
                    special_tokens.push(c.to_string());
                }
            } else if !special_tokens.is_empty() {
                special_tokens.push(c.to_string());
            } else {
                tokens.push(c.to_string());
            }
        }
        if !special_tokens.is_empty() {
            tokens.push(special_tokens.join(""));
        }
        tokens
    }
    pub fn entity_parser(text: String) -> String {
        Self::tokenize_string(&text).join("")
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::entity_parser("&amp; is an HTML entity but &ambassador; is not.".to_string())
    );
    println!(
        "{}",
        Solution::entity_parser("and I quote: &quot;...&quot;".to_string())
    );
    println!(
        "{}",
        Solution::entity_parser("and I quote: &quot;...&quotquot;".to_string())
    );
    println!("{}", Solution::entity_parser("&&gt;".to_string()));
}

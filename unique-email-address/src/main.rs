use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set = HashSet::new();
        for email in emails {
            let parts: Vec<&str> = email.split('@').collect();
            let mut local = parts[0].to_string();
            let domain = parts[1].to_string();
            local = local.replace(".", "");
            local = local.split('+').next().unwrap().to_string();
            set.insert(local + "@" + &domain);
        }
        set.len() as i32
    }
}

struct Solution;

fn stringify(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::num_unique_emails(stringify(vec![
            "test.email+alex@leetcode.com",
            "test.e.mail+bob.cathy@leetcode.com",
            "testemail+david@lee.tcode.com"
        ]))
    );
    println!(
        "{}",
        Solution::num_unique_emails(stringify(vec![
            "a@leetcode.com",
            "b@leetcode.com",
            "c@leetcode.com"
        ]))
    );
}

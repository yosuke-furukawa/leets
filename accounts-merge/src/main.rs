use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn helper(
        id_map: &HashMap<String, Vec<usize>>,
        mail_map: &HashMap<usize, Vec<String>>,
        visited: &mut HashSet<usize>,
        index: usize,
        set: &mut HashSet<String>,
    ) {
        if visited.contains(&index) {
            return;
        }
        visited.insert(index);
        for mail in mail_map.get(&index).unwrap() {
            if !set.contains(mail) {
                set.insert(mail.to_string());
                for key in id_map.get(mail).unwrap() {
                    Self::helper(id_map, mail_map, visited, *key, set);
                }
            }
        }
    }
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut id_map = HashMap::new();
        let mut mail_map: HashMap<usize, Vec<String>> = HashMap::new();
        for (id, account) in accounts.iter().enumerate() {
            let mails: Vec<String> = account.iter().skip(1).map(|s| s.to_string()).collect();
            mail_map.insert(id, mails.clone());
            for mail in mails {
                id_map.entry(mail).or_insert_with(Vec::new).push(id);
            }
        }

        let mut results = vec![];
        let mut visited = HashSet::new();
        for (i, account) in accounts.iter().enumerate() {
            let mut set = HashSet::new();
            Self::helper(&id_map, &mail_map, &mut visited, i, &mut set);
            if set.is_empty() {
                continue;
            }
            let mut result = vec![account[0].clone()];
            let mut mails: Vec<String> = set.into_iter().collect();
            mails.sort_unstable();
            result.extend(mails);
            results.push(result);
        }
        results
    }
}

struct Solution;

macro_rules! grid {
    ( $([$( $x:expr ),*]),* ) => {
        {
            vec![
                $(
                    vec![$($x.to_string()), *],
                )*
            ]
        }
    };
}

fn main() {
    println!(
        "{:?}",
        Solution::accounts_merge(grid![
            ["John", "johnsmith@mail.com", "john_newyork@mail.com"],
            ["John", "johnsmith@mail.com", "john00@mail.com"],
            ["Mary", "mary@mail.com"],
            ["John", "johnnybravo@mail.com"]
        ])
    );
}

use std::collections::HashMap;

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut result = vec![];
        let mut dict: HashMap<String, Vec<String>> = HashMap::new();
        for path in paths {
            let p: Vec<&str> = path.split(' ').collect();
            let mut dir: String = p[0].to_string();
            if !dir.ends_with('/') {
                dir += "/";
            }
            for file in p.into_iter().skip(1) {
                let f = file.to_string();
                if let Some(content_index) = f.rfind('(') {
                    let (filename, content) = f.split_at(content_index);
                    dict.entry(content.to_string())
                        .or_insert_with(Vec::new)
                        .push(dir.clone() + filename);
                }
            }
        }
        for v in dict.values().cloned() {
            if v.len() > 1 {
                result.push(v);
            }
        }
        result
    }
}

struct Solution;

fn stringify(v: Vec<&str>) -> Vec<String> {
    v.into_iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{:?}",
        Solution::find_duplicate(stringify(vec![
            "root/a 1.txt(abcd) 2.txt(efgh)",
            "root/c 3.txt(abcd)",
            "root/c/d 4.txt(efgh)",
            "root 4.txt(efgh)"
        ]))
    );
    println!(
        "{:?}",
        Solution::find_duplicate(stringify(vec![
            "root/a 1.txt(abcd) 2.txt(efsfgh)",
            "root/c 3.txt(abdfcd)",
            "root/c/d 4.txt(efggdfh)"
        ]))
    );
}

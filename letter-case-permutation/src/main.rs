impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut results = vec![];
        for c in s.chars() {
            let mut added = vec![];
            if c.is_alphabetic() {
                added.push(c.to_lowercase().to_string());
                added.push(c.to_uppercase().to_string());
            } else {
                added.push(c.to_string());
            }

            if results.is_empty() {
                for a in added.iter() {
                    results.push(a.clone());
                }
            } else {
                let mut temp = vec![];
                for r in results {
                    for a in added.iter() {
                        let t = r.clone() + a.as_str();
                        temp.push(t);
                    }
                }
                results = temp;
            }
        }
        results
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::letter_case_permutation("a1b2".to_string())
    );
}

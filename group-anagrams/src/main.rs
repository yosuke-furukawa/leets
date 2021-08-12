use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
        for str in strs {
            let mut alphas = vec![0; 26];
            for c in str.chars() {
                let a = c as u32 - 'a' as u32;
                alphas[a as usize] += 1;
            }
            result.entry(alphas).or_insert_with(Vec::new).push(str);
        }
        result.values().map(|x| x.to_vec()).collect()
    }
}

struct Solution;

fn stringify(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

fn main() {
    println!(
        "{:?}",
        Solution::group_anagrams(stringify(vec!["eat", "tea", "tan", "ate", "nat", "bat"]))
    );
}

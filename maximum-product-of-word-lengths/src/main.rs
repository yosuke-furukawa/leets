use std::collections::HashSet;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let n = words.len();
        let sets: Vec<HashSet<char>> = words
            .clone()
            .into_iter()
            .map(|word| {
                let mut set = HashSet::new();
                for c in word.chars() {
                    set.insert(c);
                }
                set
            })
            .collect();

        let mut product = 0;
        for i in 0..n {
            let wlen = words[i].len();
            for j in i..n {
                if sets[i].intersection(&sets[j]).count() == 0 {
                    product = product.max(wlen * words[j].len());
                }
            }
        }
        product as i32
    }
}

struct Solution;

fn stringify(w: Vec<&str>) -> Vec<String> {
    w.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::max_product(stringify(vec![
            "abcw", "baz", "foo", "bar", "xtfn", "abcdef"
        ]))
    );
    println!(
        "{}",
        Solution::max_product(stringify(vec!["a", "ab", "abc", "d", "cd", "bcd", "abcd"]))
    );
    println!(
        "{}",
        Solution::max_product(stringify(vec!["a", "aa", "aaa", "aaaa"]))
    );
}

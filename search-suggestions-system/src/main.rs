use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    word: String,
    candidates: Vec<String>,
    children: HashMap<char, TrieNode>,
}

#[derive(Default, Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }
    fn moving<T>(t: T) -> T {
        t
    }
    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = Trie::moving(node)
                .children
                .entry(c)
                .or_insert_with(|| TrieNode {
                    candidates: vec![],
                    ..Default::default()
                });
            node.candidates.push(word.clone());
        }
        node.word = word;
    }
    fn starts_with(&self, word: String) -> Vec<String> {
        let mut node = &self.root;
        for c in word.chars() {
            if let Some(n) = node.children.get(&c) {
                node = n;
            } else {
                return vec![];
            }
        }
        node.candidates.clone()
    }
}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut trie = Trie::new();
        let mut products = products;
        products.sort_unstable();
        for product in products {
            trie.insert(product);
        }

        let mut word = String::new();
        let mut result = vec![];
        for c in search_word.chars() {
            word += c.to_string().as_str();
            let candidates = trie.starts_with(word.clone());
            result.push(candidates.into_iter().take(3).collect());
        }
        result
    }
}

struct Solution;

fn stringify(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{:?}",
        Solution::suggested_products(
            stringify(vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"]),
            "mouse".to_string()
        )
    );
    println!(
        "{:?}",
        Solution::suggested_products(stringify(vec!["havana"]), "havana".to_string())
    );
    println!(
        "{:?}",
        Solution::suggested_products(
            stringify(vec!["bags", "baggage", "banner", "box", "cloths"]),
            "bags".to_string()
        )
    );
    println!(
        "{:?}",
        Solution::suggested_products(stringify(vec!["havana"]), "tatiana".to_string())
    );
    println!(
        "{:?}",
        Solution::suggested_products(
            stringify(vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"]),
            "mouseaa".to_string()
        )
    );
}

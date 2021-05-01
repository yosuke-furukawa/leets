use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    word: String,
    number: i32,
    children: HashMap<char, TrieNode>,
}

#[derive(Default, Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::default()
        }
    }
    fn moving<T>(t: T) -> T {
        t
    }
    fn insert(&mut self, word: String, number: i32) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = Trie::moving(node)
                .children
                .entry(c)
                .or_insert_with(|| {
                    let mut t = TrieNode::default();
                    t.number = number;
                    t
                });
        }
        node.word = word;
        node.number = number;
    }
    fn starts_with(&self, word: String) -> i32 {
        let mut node = &self.root;
        for c in word.chars() {
            if let Some(n) = node.children.get(&c) {
                node = n;
            } else {
                return -1;
            }
        }
        node.number
    }
}

struct WordFilter {
    dict: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for (i, word) in words.iter().rev().enumerate() {
            let mut key = "#".to_string() + word.as_str();
            trie.insert(key.clone(), (words.len() - i - 1) as i32);
            for c in word.chars().rev() {
                key = c.to_string() + key.as_str();
                trie.insert(key.clone(), (words.len() - i - 1) as i32);
            }
        }
        Self { dict: trie }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        let key = suffix + "#" + prefix.as_str();
        self.dict.starts_with(key)
    }
}

fn main() {
    let obj = WordFilter::new(vec!["apple".to_string()]);
    let ret: i32 = obj.f("a".to_string(), "e".to_string());
    println!("{}", ret);
}

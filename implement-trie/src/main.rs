use std::collections::HashMap;

#[derive(Clone)]
struct TrieNode {
    children: HashMap<char, Self>,
    word: String,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            word: String::new(),
        }
    }
}

struct Trie {
    root: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn moving<T>(t: T) -> T {
        t
    }

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut r = &mut self.root;
        for c in word.chars() {
            r = Trie::moving(r)
                .children
                .entry(c)
                .or_insert_with(TrieNode::new)
        }
        r.word = word;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut r = &self.root;
        for c in word.chars() {
            if let Some(n) = r.children.get(&c) {
                r = n;
            } else {
                return false;
            }
        }
        r.word == word
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut r = &self.root;
        for c in prefix.chars() {
            if let Some(n) = r.children.get(&c) {
                r = n;
            } else {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut obj = Trie::new();
    obj.insert("apple".to_string());
    let ret_2: bool = obj.search("apple".to_string());
    let ret_3: bool = obj.starts_with("app".to_string());
    println!("{}, {}", ret_2, ret_3);
}

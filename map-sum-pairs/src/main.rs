use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    word: String,
    value: HashMap<String, i32>,
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
    pub fn insert(&mut self, word: String, value: i32) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = Trie::moving(node)
                .children
                .entry(c)
                .or_insert_with(|| TrieNode {
                    value: HashMap::new(),
                    ..Default::default()
                });
            node.value.insert(word.clone(), value);
        }
        node.word = word;
    }
    pub fn starts_with(&self, word: String) -> Option<HashMap<String, i32>> {
        let mut node = &self.root;
        for c in word.chars() {
            if let Some(n) = node.children.get(&c) {
                node = n;
            } else {
                return None;
            }
        }
        Some(node.value.clone())
    }
}

struct MapSum {
    root: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MapSum { root: Trie::new() }
    }

    fn insert(&mut self, key: String, val: i32) {
        self.root.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        if let Some(map) = self.root.starts_with(prefix) {
            return map.values().sum::<i32>();
        }
        0
    }
}

fn main() {
    let mut obj = MapSum::new();
    obj.insert("apple".to_string(), 3);
    println!("{}", obj.sum("ap".to_string()));
    obj.insert("app".to_string(), 2);
    println!("{}", obj.sum("ap".to_string()));
}

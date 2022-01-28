use std::collections::HashMap;

#[derive(PartialEq, Eq, Default, Debug)]
struct Trie {
    children: HashMap<char, Trie>,
    end: bool,
}

impl Trie {
    fn insert(&mut self, s: &str) {
        let mut link = self;
        for c in s.chars() {
            link = link.children.entry(c).or_default();
        }
        link.end = true;
    }

    fn search(&self, s: &str) -> bool {
        if s.is_empty() {
            return self.end;
        }
        let c = s.chars().next().unwrap();
        if c == '.' {
            for child in self.children.values() {
                if Self::search(child, &s[1..]) {
                    return true;
                }
            }
        } else if let Some(child) = self.children.get(&c) {
            return Self::search(child, &s[1..]);
        } else {
            return false;
        }
        false
    }
}

#[derive(Default)]
struct WordDictionary {
    trie: Trie,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            trie: Trie::default(),
        }
    }
    fn add_word(&mut self, word: String) {
        self.trie.insert(&word);
    }
    fn search(&self, word: String) -> bool {
        self.trie.search(&word)
    }
}

fn main() {
    let mut obj = WordDictionary::new();
    obj.add_word("bad".to_string());
    obj.add_word("dad".to_string());
    obj.add_word("mad".to_string());
    obj.add_word("pad".to_string());
    println!("{}", obj.search("pad".to_string()));
    println!("{}", obj.search("bad".to_string()));
    println!("{}", obj.search(".ad".to_string()));
    println!("{}", obj.search("b..".to_string()));
}

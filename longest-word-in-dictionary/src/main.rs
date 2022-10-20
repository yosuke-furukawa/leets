use std::collections::HashMap;

#[derive(Clone, Default, Debug)]
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

#[derive(Clone, Default, Debug)]
struct Trie {
    root: TrieNode,
}
impl Trie {
    fn moving<T>(t: T) -> T {
        t
    }
    fn new() -> Self {
        Default::default()
    }
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
}

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut trie = Trie::new();
        let mut words = words;
        words.sort_unstable();
        for word in words.clone().into_iter() {
            trie.insert(word);
        }
        let mut longest = String::new();
        words.sort_unstable_by_key(|word| std::cmp::Reverse(word.len()));
        for word in words {
            let mut str = "".to_string();

            for c in word.chars() {
                str.push(c);
                if !trie.search(str.clone()) {
                    break;
                }
                if trie.search(str.clone()) && str.len() >= longest.len() {
                    if str.len() > longest.len() {
                        longest = str.clone();
                    } else {
                        longest = str.clone().min(longest).to_string();
                    }
                }
            }
        }
        longest
    }
}

struct Solution;

fn stringify(s: Vec<&str>) -> Vec<String> {
    s.iter().map(|x| x.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::longest_word(stringify(vec!["w", "wo", "wor", "worl", "world"]))
    );
    println!(
        "{}",
        Solution::longest_word(stringify(vec![
            "a", "banana", "app", "appl", "ap", "apply", "apple"
        ]))
    );
    println!(
        "{}",
        Solution::longest_word(stringify(vec![
            "k", "lg", "it", "oidd", "oid", "oiddm", "kfk", "y", "mw", "kf", "l", "o", "mwaqz",
            "oi", "ych", "m", "mwa"
        ]))
    );
}

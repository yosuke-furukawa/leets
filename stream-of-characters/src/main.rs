#[derive(Debug, Clone)]
struct Trie {
    is_word: bool,
    next: Vec<Option<Trie>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            is_word: false,
            next: vec![None; 26],
        }
    }
}

struct StreamChecker {
    root: Trie,
    s: String,
    max: usize,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut root = Trie::new();
        let mut max = 0;
        for w in words {
            if w.len() > max {
                max = w.len()
            }
            let v = w.chars().rev().collect::<String>();
            let mut node = &mut root;
            for ch in v.chars() {
                if node.next[(ch as u8 - b'a') as usize].is_none() {
                    node.next[(ch as u8 - b'a') as usize] = Some(Trie::new())
                }
                node = node.next[(ch as u8 - b'a') as usize].as_mut().unwrap();
            }
            node.is_word = true;
        }
        let s = String::new();
        StreamChecker { root, s, max }
    }

    fn query(&mut self, letter: char) -> bool {
        self.s.push(letter);
        let mut node = &mut self.root;
        let mut start = 0;
        if self.s.len() > self.max {
            start = self.s.len() - self.max;
        };
        for i in (start..self.s.len()).rev() {
            let ch = self.s.as_bytes()[i] as char;
            if let Some(n) = &mut node.next[(ch as u8 - b'a') as usize] {
                if n.is_word {
                    return true;
                }
                node = n
            } else {
                return false;
            }
        }
        false
    }
}

/**
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */

fn main() {
    let mut obj = StreamChecker::new(vec!["cd".to_string(), "f".to_string(), "kl".to_string()]);
    println!("{}", obj.query('a'));
    println!("{}", obj.query('b'));
    println!("{}", obj.query('c'));
    println!("{}", obj.query('d'));
}

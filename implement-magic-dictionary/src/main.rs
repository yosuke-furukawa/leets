struct MagicDictionary {
    root: Trie,
}

impl MagicDictionary {
    fn new() -> Self {
        MagicDictionary {
            root: Default::default(),
        }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for s in dictionary {
            let mut cur: &mut Trie = &mut self.root;
            for i in s.chars().map(|ch| (ch as u8 - b'a') as usize) {
                cur = cur.children[i].get_or_insert(Box::new(Default::default()));
            }
            cur.is_end = true;
        }
    }

    fn search(&self, search_word: String) -> bool {
        let mut cur = &self.root;
        let arr: &mut Vec<_> = &mut search_word.chars().collect();
        for i in 0..arr.len() {
            for c in b'a'..=b'z' {
                if arr[i] as u8 == c {
                    continue;
                }
                let t_ch = arr[i];
                arr[i] = c as char;
                if Self::end_with(arr, cur, i) {
                    return true;
                }
                arr[i] = t_ch;
            }
            let idx = (arr[i] as u8 - b'a') as usize;
            match cur.children[idx].as_ref() {
                None => {
                    return false;
                }
                Some(next) => {
                    cur = next;
                }
            }
        }
        false
    }

    fn end_with(arr: &[char], node: &Trie, begin: usize) -> bool {
        let mut cur = node;
        for &ch in arr.iter().skip(begin) {
            let idx = (ch as u8 - b'a') as usize;
            match cur.children[idx].as_ref() {
                None => {
                    return false;
                }
                Some(next) => {
                    cur = next;
                }
            }
        }
        cur.is_end
    }
}

#[derive(Default)]
struct Trie {
    is_end: bool,
    children: [Option<Box<Trie>>; 26],
}

fn main() {
    let mut dict = MagicDictionary::new();
    dict.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);
    println!("{}", dict.search("hello".to_string()));
    println!("{}", dict.search("hhllo".to_string()));
    println!("{}", dict.search("hell".to_string()));
    println!("{}", dict.search("leetcoded".to_string()));
}

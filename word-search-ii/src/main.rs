use std::collections::HashMap;

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        let mut board = board;
        let mut trie = Trie::new();
        for word in words {
            trie.insert(word);
        }
        for col in 0..board.len() {
            for row in 0..board[col].len() {
                if trie.root.children.contains_key(&board[col][row]) {
                    Self::backtrack(&mut board, col, row, &mut trie.root, &mut result);
                }
            }
        }
        result
    }
    fn backtrack(
        board: &mut Vec<Vec<char>>,
        col: usize,
        row: usize,
        parent: &mut TrieNode,
        result: &mut Vec<String>,
    ) {
        let char = board[col][row];
        if let Some(node) = parent.children.get_mut(&char) {
            if node.word.is_some() {
                result.push(node.word.as_ref().unwrap().clone());
                node.word = None;
            }

            board[col][row] = '#';
            let row_offsets = [0, 1, 0, -1];
            let col_offsets = [1, 0, -1, 0];
            for i in 0..4 {
                let new_row = row as i32 + row_offsets[i];
                let new_col = col as i32 + col_offsets[i];
                if new_col < 0
                    || new_col as usize >= board.len()
                    || new_row < 0
                    || new_row as usize >= board[0].len()
                {
                    continue;
                }
                if node
                    .children
                    .contains_key(&board[new_col as usize][new_row as usize])
                {
                    Self::backtrack(board, new_col as usize, new_row as usize, node, result);
                }
            }
            board[col][row] = char;
            if node.children.is_empty() {
                parent.children.remove(&char);
            }
        }
    }
}

#[derive(Clone)]
struct TrieNode {
    children: HashMap<char, Self>,
    word: Option<String>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            word: None,
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
        r.word = Some(word);
    }
}

struct Solution;

macro_rules! grid {
    ( $([$( $x:expr ),*]),* ) => {
        {
            vec![$(vec![$($x), *],)*]
        }
    };
}

fn stringify(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

fn main() {
    println!(
        "{:?}",
        Solution::find_words(
            grid![
                ['o', 'a', 'a', 'n'],
                ['e', 't', 'a', 'e'],
                ['i', 'h', 'k', 'r'],
                ['i', 'f', 'l', 'v']
            ],
            stringify(vec!["oath", "pea", "eat", "rain"])
        )
    );
}

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let words_max = words.iter().fold(0, |acc, word| acc.max(word.len()));
        let mut words_matrix: Vec<Vec<String>> = vec![vec![]; words_max];
        for word in words {
            let len = word.len();
            if let Some(w) = words_matrix.get_mut(len - 1) {
                w.push(word);
            } else {
                words_matrix[len - 1] = vec![word];
            }
        }
        // println!("{:?}", words_matrix);
        let mut max = 0;
        let mut current = words_matrix.len();
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();
        let mut visited = HashSet::new();
        for i in (0..current).rev() {
            for word in words_matrix[i].iter() {
                queue.push_back((word.clone(), 1));
            }
        }
        while !queue.is_empty() {
            // println!("{:?}", queue);
            let item = queue.pop_front().unwrap();
            let prev = item.0;
            let prev_len = prev.len();
            current = current.min(prev_len);
            if visited.contains(&prev) {
                continue;
            }
            visited.insert(prev.clone());
            max = max.max(item.1);
            if prev_len < 2 {
                continue;
            }
            for word in words_matrix[prev_len - 2].iter() {
                let mut diff = 0;
                for (i, p) in prev.chars().enumerate() {
                    let word_index = i - diff;
                    if let Some(w) = word.get(word_index..word_index + 1) {
                        if p.to_string() != w {
                            if diff == 1 {
                                diff += 1;
                                break;
                            }
                            diff += 1;
                        }
                    }
                }
                if diff <= 1 {
                    queue.push_front((word.clone(), item.1 + 1));
                }
            }
        }
        max
    }
}

struct Solution;

fn stringify(words: Vec<&str>) -> Vec<String> {
    words.iter().map(|word| word.to_string()).collect()
}

fn main() {
    // println!("{}", Solution::longest_str_chain(stringify(vec!["a","b","ba","bca","bda","ddd","bdca"])));
    println!(
        "{}",
        Solution::longest_str_chain(stringify(vec![
            "qyssedya",
            "pabouk",
            "mjwdrbqwp",
            "vylodpmwp",
            "nfyqeowa",
            "pu",
            "paboukc",
            "qssedya",
            "lopmw",
            "nfyqowa",
            "vlodpmw",
            "mwdrqwp",
            "opmw",
            "qsda",
            "neo",
            "qyssedhyac",
            "pmw",
            "lodpmw",
            "mjwdrqwp",
            "eo",
            "nfqwa",
            "pabuk",
            "nfyqwa",
            "qssdya",
            "qsdya",
            "qyssedhya",
            "pabu",
            "nqwa",
            "pabqoukc",
            "pbu",
            "mw",
            "vlodpmwp",
            "x",
            "xr"
        ]))
    );
}

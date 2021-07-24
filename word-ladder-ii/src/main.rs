use std::collections::HashMap;
use std::collections::VecDeque;
use std::i32::MAX;

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut word_id = HashMap::new();
        let mut id_word = vec![];
        let mut id = 0;

        for word in word_list.iter() {
            if !word_id.contains_key(word) {
                word_id.insert(word.to_string(), id);
                id += 1;
                id_word.push(word.to_string());
            }
        }

        if !word_id.contains_key(&end_word) {
            return vec![];
        }

        if !word_id.contains_key(&begin_word) {
            word_id.insert(begin_word.to_string(), id);
            id += 1;
            id_word.push(begin_word.to_string());
        }
        let mut edges = vec![vec![]; id_word.len()];

        for i in 0..id_word.len() {
            for j in (i + 1)..id_word.len() {
                if Self::transform_check(
                    id_word.get(i).unwrap().as_str(),
                    id_word.get(j).unwrap().as_str(),
                ) {
                    edges[i].push(j);
                    edges[j].push(i);
                }
            }
        }

        let dest = *word_id.get(&end_word).unwrap();
        let begin = *word_id.get(&begin_word).unwrap();

        Solution::bfs(id, begin, dest, id_word, edges)
    }

    fn bfs(
        id: usize,
        begin: usize,
        dest: usize,
        dict: Vec<String>,
        edges: Vec<Vec<usize>>,
    ) -> Vec<Vec<String>> {
        let mut ans = vec![];
        let mut cost = vec![MAX; id];

        let mut queue = VecDeque::new();
        queue.push_back(vec![begin]);
        cost[begin] = 0;

        while let Some(now) = queue.pop_front() {
            if let Some(last) = now.last() {
                if *last == dest {
                    let mut tmp = vec![];
                    for index in now.iter() {
                        tmp.push(dict[*index as usize].to_string());
                    }

                    ans.push(tmp);
                } else {
                    for i in 0..edges[*last as usize].len() {
                        let to = *edges[*last as usize].get(i).unwrap();
                        if cost[*last as usize] < cost[to as usize] {
                            cost[to as usize] = cost[*last as usize] + 1;
                            let mut tmp = vec![];
                            tmp.extend(now.iter().copied());
                            tmp.push(to);
                            queue.push_back(tmp);
                        }
                    }
                }
            }
        }

        ans
    }

    fn transform_check(s1: &str, s2: &str) -> bool {
        let mut diff = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                diff += 1;
            }
        }

        diff == 1
    }
}

struct Solution;

fn stringify(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

fn main() {
    println!(
        "{:?}",
        Solution::find_ladders(
            "hit".to_string(),
            "cog".to_string(),
            stringify(vec!["hot", "dot", "dog", "lot", "log", "cog"])
        )
    );
}

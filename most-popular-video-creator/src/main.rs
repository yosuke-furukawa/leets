use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn most_popular_creator(
        creators: Vec<String>,
        ids: Vec<String>,
        views: Vec<i32>,
    ) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        let mut score_map = HashMap::new();
        let mut id_map = HashMap::new();
        let mut max = 0;

        for (i, creator) in creators.iter().enumerate() {
            let entry = map.entry(creator).or_insert(0);
            *entry += views[i];
            max = max.max(*entry);
            score_map
                .entry(*entry)
                .or_insert_with(HashSet::new)
                .insert(creator);
        }
        for (i, creator) in creators.iter().enumerate() {
            let entry = id_map.entry(creator).or_insert(BTreeSet::new());
            entry.insert(Reverse((views[i], Reverse(&ids[i]))));
        }
        let creators = score_map.get(&max).unwrap();
        let mut results = vec![];
        for creator in creators {
            let mut result = vec![];
            if let Some(videos) = id_map.get(&creator.to_string()) {
                if let Some(Reverse((_view, Reverse(id)))) = videos.iter().next() {
                    result.push(creator.to_string());
                    result.push(id.to_string());
                }
            }
            results.push(result);
        }
        results
    }
}

struct Solution;

fn stringify(s: Vec<&str>) -> Vec<String> {
    s.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{:?}",
        Solution::most_popular_creator(
            stringify(vec!["alice", "bob", "alice", "chris"]),
            stringify(vec!["one", "two", "three", "four"]),
            vec![5, 10, 5, 4]
        )
    );
    println!(
        "{:?}",
        Solution::most_popular_creator(
            stringify(vec!["alice", "alice", "alice"]),
            stringify(vec!["a", "b", "c"]),
            vec![1, 2, 2]
        )
    );
}

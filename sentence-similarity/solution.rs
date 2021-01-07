use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::HashSet;

impl Solution {
    pub fn are_sentences_similar(sentence1: Vec<String>, sentence2: Vec<String>, similar_pairs: Vec<Vec<String>>) -> bool {
      let mut map = HashMap::new();
      
      for pair in similar_pairs.iter() {
        let v1 = pair.first().unwrap();
        let v2 = pair.last().unwrap();
        map.entry(v1).or_insert(HashSet::new()).insert(v2);
        map.entry(v2).or_insert(HashSet::new()).insert(v1);
      }
      
      if sentence1.len() != sentence2.len() {
        return false;
      }
      
      for i in 0..sentence1.len() {
        if (sentence1[i] == sentence2[i]) {
          continue;
        }
        if (!map.contains_key(&sentence1[i])) {
          return false;
        }
        if (!map.get(&sentence1[i]).unwrap().contains(&sentence2[i])) {
          return false;
        }
      }
      
      return true;
    }
}

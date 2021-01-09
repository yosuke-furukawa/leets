use std::collections::HashMap;

fn anagram_mappings(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, v) in b.iter().enumerate() {
        map.insert(v, i as i32);
    }
    let mut result: Vec<i32> = vec![];
    for v in a {
        if map.contains_key(&v) {
            result.push(*map.get(&v).unwrap());
        }
    }
    return result;
}

fn main() {
    let answer = anagram_mappings(vec![12, 28, 46, 32, 50], vec![50, 12, 32, 46, 28]);
    println!("{:?}", answer);
}

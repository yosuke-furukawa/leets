use std::collections::HashMap;

impl Solution {
    fn create_counts(s: String) -> HashMap<char, i32> {
        s.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
    }
    pub fn original_digits(s: String) -> String {
        let mut words: Vec<char> = s.chars().collect();
        let mut nums: HashMap<char, (i32, HashMap<char, i32>)> = HashMap::new();
        nums.insert('z', (0, Self::create_counts("ero".to_string())));
        nums.insert('o', (1, Self::create_counts("ne".to_string())));
        nums.insert('w', (2, Self::create_counts("to".to_string())));
        nums.insert('r', (3, Self::create_counts("thee".to_string())));
        nums.insert('u', (4, Self::create_counts("for".to_string())));
        nums.insert('f', (5, Self::create_counts("ive".to_string())));
        nums.insert('x', (6, Self::create_counts("si".to_string())));
        nums.insert('s', (7, Self::create_counts("even".to_string())));
        nums.insert('g', (8, Self::create_counts("eiht".to_string())));
        nums.insert('n', (9, Self::create_counts("ine".to_string())));
        words.sort_unstable_by(|a, b| b.cmp(&a));
        let mut skip_map: HashMap<char, i32> = HashMap::new();
        let mut n: Vec<i32> = vec![];
        for c in words.iter() {
            if let Some(skipped) = skip_map.get_mut(c) {
                if *skipped > 0 {
                    *skipped -= 1;
                    continue;
                }
            }
            if let Some((num, skips)) = nums.get(c) {
                for (key, value) in skips {
                    *skip_map.entry(*key).or_insert(0) += value;
                }
                n.push(*num);
            }
        }
        n.sort_unstable();
        n.iter().map(|x| x.to_string()).collect()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::original_digits("owoztneoer".to_string()));
    println!("{}", Solution::original_digits("fviefuro".to_string()));
}

use std::collections::HashMap;
struct Solution {}

impl Solution {
    pub fn high_five(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut highmap: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut result = vec![];

        for (id, score) in items.iter().map(|v| (v[0], v[1])) {
            let mut scores = highmap.entry(id).or_insert(vec![]).clone();

            if scores.len() < 5 {
                scores.push(score);
                highmap.insert(id, scores);
            } else if scores.len() == 5 {
                scores.sort_by(|a, b| b.cmp(a));
                let f = scores.first().unwrap();
                let l = scores.last().unwrap();
                if f < &score || l < &score && f >= &score {
                    scores.pop();
                    scores.push(score);
                    highmap.insert(id, scores);
                }
            }
        }

        for (id, scores) in highmap.iter() {
            let avg = scores.iter().sum::<i32>() / 5;
            result.push(vec![*id, avg]);
        }

        result
    }
}

fn main() {
    let items = [
        [1, 91],
        [1, 92],
        [2, 93],
        [2, 97],
        [1, 60],
        [2, 77],
        [1, 65],
        [1, 87],
        [1, 100],
        [2, 100],
        [2, 76],
    ];
    let input = items.iter().map(|m| m.to_vec()).collect();
    println!("{:?}", Solution::high_five(input));
}

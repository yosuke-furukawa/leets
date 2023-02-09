use std::collections::HashSet;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut a: [HashSet<_>; 26] = Default::default();
        for idea in &ideas {
            a[(idea.as_bytes()[0] - b'a') as usize].insert(&idea[1..]);
        }
        a.iter()
            .enumerate()
            .flat_map(|(i, ai)| a[i + 1..].iter().map(move |aj| (ai, aj)))
            .map(|(ai, aj)| {
                let c = ai.intersection(aj).count();
                (ai.len() - c) * (aj.len() - c)
            })
            .sum::<usize>() as i64
            * 2
    }
}

struct Solution;

fn stringify(s: Vec<&str>) -> Vec<String> {
    s.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::distinct_names(stringify(vec!["coffee", "donuts", "time", "toffee"]))
    );
}

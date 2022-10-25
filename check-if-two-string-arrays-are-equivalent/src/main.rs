impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut s1 = String::new();
        let mut s2 = String::new();
        for word in word1 {
            s1.push_str(&word);
        }
        for word in word2 {
            s2.push_str(&word);
        }
        s1 == s2
    }
}

struct Solution;

fn stringify(s: Vec<&str>) -> Vec<String> {
    s.into_iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::array_strings_are_equal(stringify(vec!["ab", "c"]), stringify(vec!["a", "bc"]))
    );
}

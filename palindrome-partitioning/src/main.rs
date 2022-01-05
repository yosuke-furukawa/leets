impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = vec![];
        let s: Vec<char> = s.chars().collect();
        Self::dfs_helper(&s, &mut vec![], &mut res, 0);
        res
    }

    fn dfs_helper(s: &[char], cans: &mut Vec<String>, res: &mut Vec<Vec<String>>, idx: usize) {
        if idx == s.len() {
            res.push(cans.clone());
        }

        for i in idx..s.len() {
            if !Self::is_palindrome(s, idx, i) {
                continue;
            }
            let can: String = s[idx..i + 1].iter().collect();
            cans.push(can);
            Self::dfs_helper(s, cans, res, i + 1);
            cans.pop();
        }
    }

    fn is_palindrome(s: &[char], start: usize, end: usize) -> bool {
        let (mut start, mut end) = (start, end);
        while start < end {
            if s[start] != s[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::partition("aab".to_string()));
}

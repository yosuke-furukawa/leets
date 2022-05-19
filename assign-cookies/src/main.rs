impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        g.sort_unstable();
        s.sort_unstable();
        let mut count = 0;
        let mut gi = 0;
        let mut si = 0;
        while gi < g.len() && si < s.len() {
            if s[si] >= g[gi] {
                count += 1;
                gi += 1;
                si += 1;
            } else {
                si += 1;
            }
        }
        count
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::find_content_children(vec![1, 2, 3], vec![1, 1])
    );
    println!(
        "{}",
        Solution::find_content_children(vec![1, 2], vec![1, 2, 3])
    );
    println!("{}", Solution::find_content_children(vec![1], vec![1]));
}

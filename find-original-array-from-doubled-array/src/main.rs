impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        let mut changed = changed;
        changed.sort_unstable();
        let mut ans = vec![];
        while let Some(x) = changed.pop() {
            if x % 2 == 1 {
                return vec![];
            }
            if let Ok(i) = changed.binary_search(&(x / 2)) {
                ans.push(x / 2);
                changed.remove(i);
            } else {
                return vec![];
            }
        }
        ans
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::find_original_array(vec![1, 3, 4, 2, 6, 8])
    );
}

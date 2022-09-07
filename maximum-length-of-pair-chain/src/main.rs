impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs;
        pairs.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut ans = 0;
        let mut cur = std::i32::MIN;
        for pair in pairs {
            if cur < pair[0] {
                ans += 1;
                cur = pair[1];
            }
        }
        ans
    }
}

struct Solution;

macro_rules! grid {
    ( $([$( $x:expr ),*]),* ) => {
        {
            vec![
                $(
                    vec![$($x), *],
                )*
            ]
        }
    };
}

fn main() {
    println!(
        "{}",
        Solution::find_longest_chain(grid![[1, 2], [2, 3], [3, 4]])
    );
    println!(
        "{}",
        Solution::find_longest_chain(grid![[1, 2], [7, 8], [4, 5]])
    );
}

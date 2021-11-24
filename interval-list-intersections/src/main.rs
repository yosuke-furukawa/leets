impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut results = vec![];
        let mut fi = 0;
        let mut si = 0;
        while fi < first_list.len() && si < second_list.len() {
            let f = &first_list[fi];
            let s = &second_list[si];

            if f[0] <= s[1] && f[1] >= s[0] {
                results.push(vec![f[0].max(s[0]), f[1].min(s[1])]);
            }

            if f[1] < s[1] {
                fi += 1;
            } else {
                si += 1;
            }
        }
        results
    }
}

struct Solution;

#[macro_export]
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
        "{:?}",
        Solution::interval_intersection(
            grid![[0, 2], [5, 10], [13, 23], [24, 25]],
            grid![[1, 5], [8, 12], [15, 24], [25, 26]]
        )
    );
}

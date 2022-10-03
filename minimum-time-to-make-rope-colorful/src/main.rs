impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut needed_time = needed_time;
        let colors: Vec<char> = colors.chars().collect();
        let mut total = 0;
        for i in 1..colors.len() {
            if colors[i] == colors[i - 1] {
                total += needed_time[i - 1].min(needed_time[i]);
                if needed_time[i - 1] > needed_time[i] {
                    needed_time.swap(i - 1, i);
                }
            }
        }
        total
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5])
    );
}

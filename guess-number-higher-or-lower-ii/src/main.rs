impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut table = vec![vec![0i32; n + 1]; n + 1];

        for di in 1..n {
            for i in 1..(n + 1 - di) {
                let j = i + di;
                let pivots = (i + di / 2)..j;
                let pivot_costs = pivots.map(|p| p as i32 + table[i][p - 1].max(table[p + 1][j]));
                table[i][j] = pivot_costs.min().unwrap_or(0);
            }
        }

        table[1][n]
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::get_money_amount(10));
}

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut evens = 0;
        let mut odds = 0;
        for n in position {
            if n % 2 == 0 {
                evens += 1;
                continue;
            }
            odds += 1;
        }
        evens.min(odds)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::min_cost_to_move_chips(vec![1, 2, 3]));
}

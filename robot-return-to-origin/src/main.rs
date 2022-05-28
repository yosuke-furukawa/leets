impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        for c in moves.chars() {
            match c {
                'L' => x -= 1,
                'R' => x += 1,
                'U' => y += 1,
                'D' => y -= 1,
                _ => panic!("no such move {}", c),
            }
        }
        x == 0 && y == 0
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::judge_circle(String::from("UD")));
}

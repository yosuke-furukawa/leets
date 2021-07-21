impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let chars: Vec<char> = dominoes.chars().collect();
        let n = chars.len();
        let mut forces: Vec<i32> = vec![0; n];

        let mut force = 0;
        for i in 0..n {
            force = match chars[i] {
                'R' => n as i32,
                'L' => 0,
                _ => (force - 1).max(0),
            };
            forces[i] += force;
        }

        force = 0;
        for i in (0..n).rev() {
            force = match chars[i] {
                'L' => n as i32,
                'R' => 0,
                _ => (force - 1).max(0),
            };
            forces[i] -= force;
        }

        let mut ans = String::new();
        for f in forces {
            ans += match f {
                _ if f > 0 => "R",
                _ if f < 0 => "L",
                _ => ".",
            };
        }
        ans
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::push_dominoes("RR.L".to_string()));
    println!("{}", Solution::push_dominoes(".L.R...LR..L..".to_string()));
}

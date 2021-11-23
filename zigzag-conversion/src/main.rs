impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut matrix = vec![vec!['-'; s.len()]; num_rows as usize];
        let mut direction = (1, 0);
        let mut px = 0;
        let mut py = 0;
        for ch in s.chars() {
            matrix[px as usize][py as usize] = ch;
            if px == 0 {
                direction = (1, 0);
            }
            if px == num_rows - 1 {
                direction = (-1, 1);
            }
            px += direction.0;
            if px < 0 {
                px = 0;
            }
            py += direction.1;
        }
        let mut result = String::new();
        for m in matrix {
            for n in m {
                if n != '-' {
                    result += &n.to_string();
                }
            }
        }
        result
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::convert("PAYPALISHIRING".to_string(), 3));
    println!("{}", Solution::convert("AB".to_string(), 1));
}

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut lines = 1;
        let mut last_line = 0;
        for c in s.chars() {
            let w = widths[(c as u8 - b'a') as usize];
            if last_line + w > 100 {
                lines += 1;
                last_line = w;
            } else {
                last_line += w;
            }
        }
        vec![lines, last_line]
    }
}

struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::number_of_lines(
            vec![
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        )
    );
}

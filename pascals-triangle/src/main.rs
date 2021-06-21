impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut rows = vec![];
        for i in 0..num_rows as usize {
            if i == 0 {
                rows.push(vec![1]);
                continue;
            }
            let mut r = vec![];
            for j in 0..=i {
                let n = match j {
                    _ if j > 0 && j < rows[i - 1].len() => rows[i - 1][j - 1] + rows[i - 1][j],
                    _ => 1,
                };
                r.push(n);
            }
            rows.push(r);
        }
        rows
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::generate(5));
}

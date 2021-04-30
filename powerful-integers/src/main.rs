use std::collections::HashSet;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut x_max: i32 = (bound as f64).log(x as f64) as i32;
        let mut y_max: i32 = (bound as f64).log(y as f64) as i32;
        if x_max < 0 {
            x_max = 1;
        }
        if y_max < 0 {
            y_max = 1;
        }
        let mut answers = HashSet::new();
        for i in 0..=x_max {
            for j in 0..=y_max {
                let ans = x.pow(i as u32) + y.pow(j as u32);
                if ans <= bound {
                    answers.insert(ans);
                } else {
                    break;
                }
            }
        }
        answers.into_iter().collect()
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::powerful_integers(2, 3, 10));
}

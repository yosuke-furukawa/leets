impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut w = (area as f64).sqrt().floor() as i32;
        while area % w != 0 {
            w -= 1;
        }
        vec![area / w, w]
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::construct_rectangle(4));
    println!("{:?}", Solution::construct_rectangle(37));
    println!("{:?}", Solution::construct_rectangle(122122));
}

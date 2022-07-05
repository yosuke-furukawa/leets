impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        let mut count = 1;
        let mut magical = vec![1, 2];
        let mut i = 1;
        while magical.len() < n as usize {
            while count < magical[i] {
                magical.push(*magical.last().unwrap());
                count += 1;
            }
            i += 1;
            count = 1;
            magical.push((magical.last().unwrap() - 3i32).abs());
        }

        magical
            .into_iter()
            .take(n as usize)
            .filter(|x| *x == 1)
            .count() as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::magical_string(6));
}

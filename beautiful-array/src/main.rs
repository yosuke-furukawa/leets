impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut v = vec![1];

        while v.len() < n as usize {
            let mut temp = vec![];
            for num in v.iter() {
                if num * 2 - 1 <= n {
                    temp.push(num * 2 - 1);
                }
            }
            for num in v.iter() {
                if num * 2 <= n {
                    temp.push(num * 2);
                }
            }

            v = temp;
        }
        v
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::beautiful_array(5));
}

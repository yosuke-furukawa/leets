impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut res = vec![];
        for h in 0..12_u32 {
            for m in 0..60_u32 {
                if h.count_ones() + m.count_ones() == turned_on as u32 {
                    res.push(format!("{}:{:02}", h, m));
                }
            }
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::read_binary_watch(1));
}

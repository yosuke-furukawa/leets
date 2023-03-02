impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut res = 0;
        let mut i = 0;
        while i < chars.len() {
            let mut group_length = 1;
            while i + group_length < chars.len() && chars[i + group_length] == chars[i] {
                group_length += 1;
            }
            chars[res] = chars[i];
            res += 1;
            if group_length > 1 {
                for ch in group_length.to_string().chars() {
                    chars[res] = ch;
                    res += 1;
                }
            }
            i += group_length;
        }
        res as i32
    }
}

struct Solution;

fn main() {
    let mut string = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    println!("{}", Solution::compress(&mut string));
    println!("{:?}", string);
}

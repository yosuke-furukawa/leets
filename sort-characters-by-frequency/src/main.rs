impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut frequency = [0; 256];
        let mut bytes = s.into_bytes();
        for &byte in &bytes {
            frequency[byte as usize] += 1;
        }
        bytes.sort_by_key(|&byte| (-frequency[byte as usize], -(byte as i16)));
        String::from_utf8(bytes).unwrap()
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::frequency_sort("tree".to_string()));
    println!("{}", Solution::frequency_sort("cccaaa".to_string()));
    println!("{}", Solution::frequency_sort("Aabb".to_string()));
}

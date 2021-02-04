impl Solution {
    pub fn smallest_equivalent_string(a: String, b: String, s: String) -> String {
        let chars_a: Vec<char> = a.chars().collect();
        let chars_b: Vec<char> = b.chars().collect();
        let mut alphabet_map = Vec::with_capacity(26);
        for i in 0..26 {
            alphabet_map.push(i);
        }

        for i in 0..chars_a.len() {
            let mut a = chars_a[i] as u8 - b'a';
            let mut b = chars_b[i] as u8 - b'a';
            while alphabet_map[b as usize] != b {
                b = alphabet_map[b as usize];
            }
            while alphabet_map[a as usize] != a {
                a = alphabet_map[a as usize];
            }
            if b < alphabet_map[a as usize] {
                alphabet_map[a as usize] = alphabet_map[b as usize];
            }
            if a < alphabet_map[b as usize] {
                alphabet_map[b as usize] = alphabet_map[a as usize];
            }
        }
        let chars_s: Vec<char> = s.chars().collect();
        let mut result: Vec<u8> = Vec::with_capacity(chars_s.len());
        for c in chars_s {
            let mut c = c as u8 - b'a';
            while alphabet_map[c as usize] != c {
                c = alphabet_map[c as usize];
            }
            result.push(alphabet_map[c as usize] + b'a');
        }
        std::str::from_utf8(&result).unwrap().to_string()
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::smallest_equivalent_string(
            "abc".to_string(),
            "cde".to_string(),
            "eed".to_string()
        )
    );
}

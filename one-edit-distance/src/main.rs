impl Solution {
    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        let slen = s.len();
        let tlen = t.len();
        if ((slen - tlen) as i32).abs() > 1 {
            return false;
        }

        let mut one_diff = false;
        if slen == tlen {
            let tchars: Vec<char> = t.chars().collect();
            for (i, v) in s.chars().enumerate() {
                if v != tchars[i] {
                    if one_diff {
                        return false;
                    }
                    one_diff = true
                }
            }
            return one_diff;
        }

        let mut short_index = 0;
        let mut short = s.clone();
        let mut long = t.clone();
        if slen > tlen {
            short = long.clone();
            long = s.clone();
        }

        let ss: Vec<char> = short.chars().collect();
        let mut one_edit = false;
        for l in long.chars() {
            if l != *ss.get(short_index).unwrap_or(&'0') {
                if one_edit {
                    return false;
                }
                one_edit = true;
                continue;
            }
            short_index += 1;
        }

        true
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::is_one_edit_distance("abc".to_string(), "def".to_string())
    );
    println!(
        "{}",
        Solution::is_one_edit_distance("abc".to_string(), "ab".to_string())
    );
}

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut tmap = HashMap::new();
        for c in t.chars() {
            *tmap.entry(c).or_insert(0) += 1;
        }
        let required = tmap.len();
        let mut uniq_chars = 0;
        let mut left = 0;
        let mut right = 0;
        let mut fixed_left = 0;
        let mut fixed_right = 0;
        let mut len = 0;
        let mut smap = HashMap::new();

        while let Some(st) = s.get(right..right + 1) {
            let c = st.chars().next().unwrap();
            *smap.entry(c).or_insert(0) += 1;
            if tmap.contains_key(&c) && tmap.get(&c) == smap.get(&c) {
                uniq_chars += 1;
            }

            while left <= right && uniq_chars == required {
                let c = s.get(left..left + 1).unwrap().chars().next().unwrap();
                if len == 0 || right - left + 1 < len {
                    len = right - left + 1;
                    fixed_left = left;
                    fixed_right = right;
                }
                if let Some(count) = smap.get_mut(&c) {
                    *count -= 1;
                    if tmap.contains_key(&c) && smap.get(&c) < tmap.get(&c) {
                        uniq_chars -= 1;
                    }
                }
                left += 1;
            }

            right += 1;
        }

        if len == 0 {
            "".to_string()
        } else {
            s.get(fixed_left..=fixed_right).unwrap().to_string()
        }
    }
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string())
    );
}

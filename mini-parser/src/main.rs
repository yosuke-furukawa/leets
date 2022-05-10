impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        if !s.starts_with('[') {
            return NestedInteger::Int(s.parse::<i32>().unwrap());
        }
        Self::parse_to_list(&s.chars().collect::<Vec<char>>(), 0, None).0
    }
    fn parse_to_list(
        s: &[char],
        cursor: usize,
        prev: Option<NestedInteger>,
    ) -> (NestedInteger, usize) {
        let mut cur = cursor;
        let mut list = vec![];
        let mut item = prev;
        while cur < s.len() {
            match s[cur] {
                '[' => {
                    let (l, c) = Self::parse_to_list(s, cur + 1, item);
                    item = Some(l);
                    cur = c;
                }
                ',' => {
                    if let Some(v) = item {
                        list.push(v);
                        cur += 1;
                        item = None;
                    }
                }
                ']' => {
                    if let Some(v) = item {
                        list.push(v);
                    }
                    return (NestedInteger::List(list), cur + 1);
                }
                c if c.is_digit(10) || c == '-' => {
                    let (val, len) = Self::parse_to_int(&s[cur..]);
                    item = Some(NestedInteger::Int(val));
                    cur += len;
                }
                _ => {
                    cur += 1;
                }
            }
        }
        (item.unwrap(), cur)
    }
    fn parse_to_int(s: &[char]) -> (i32, usize) {
        let mut res = 0;
        let mut sign = 1;
        let mut len = 0;
        for c in s {
            if *c == '-' {
                sign = -1;
                len += 1;
                continue;
            }
            if !c.is_digit(10) {
                break;
            }
            res = res * 10 + c.to_digit(10).unwrap() as i32;
            len += 1;
        }
        (res * sign, len)
    }
}

struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

fn main() {
    println!(
        "{:?}",
        Solution::deserialize("[123,[456,[789]]]".to_string())
    );
    println!(
        "{:?}",
        Solution::deserialize("[123,[456,[789,1011]]]".to_string())
    );
    println!(
        "{:?}",
        Solution::deserialize("[123,[456,123,111,[789,1011]]]".to_string())
    );
    println!(
        "{:?}",
        Solution::deserialize("[123,234,[456,[1,[789,1011]]]]".to_string())
    );
    println!(
        "{:?}",
        Solution::deserialize("[123,234,[456,[1,[789,1011],2]]]".to_string())
    );
}

use std::collections::VecDeque;

impl Solution {
    fn calc(w: &[char], index: &mut usize) -> i32 {
        let mut queue = VecDeque::new();
        let mut op = vec![];
        let mut num = 0;
        while *index < w.len() {
            let t = w[*index];
            *index += 1;
            match t {
                _ if ('0'..='9').contains(&t) => num = num * 10 + (t as i32 - '0' as i32),
                '(' => {
                    num = Self::calc(w, index);
                }
                ')' => {
                    queue.push_back(num);
                    break;
                }
                '+' | '-' => {
                    queue.push_back(num);
                    num = 0;
                    op.push(t);
                }
                '\0' => {
                    queue.push_back(num);
                    break;
                }
                _ => panic!("no such char {}", t),
            }
        }
        for o in op {
            let n1 = queue.pop_front().unwrap();
            let n2 = queue.pop_front().unwrap();
            let n3 = match o {
                '+' => n1 + n2,
                '-' => n1 - n2,
                _ => panic!("no such op {}", o),
            };
            queue.push_front(n3);
        }
        *queue.get(0).unwrap()
    }
    pub fn calculate(s: String) -> i32 {
        let mut contents: Vec<char> = s.replace(" ", "").chars().collect();
        contents.push('\0');
        Self::calc(&contents, &mut 0)
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::calculate("1 + 1".to_string()));
    println!("{}", Solution::calculate(" 2-1 + 2 ".to_string()));
    println!("{}", Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()));
}

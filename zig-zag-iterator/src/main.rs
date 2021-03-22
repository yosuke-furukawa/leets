use std::collections::VecDeque;

struct ZigzagIterator {
    v1: VecDeque<i32>,
    v2: VecDeque<i32>,
    curr: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ZigzagIterator {
    /** initialize your data structure here. */

    fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
        Self {
            v1: v1.into_iter().collect(),
            v2: v2.into_iter().collect(),
            curr: 0,
        }
    }

    fn next(&mut self) -> i32 {
        let ans = match (self.v1.clone(), self.v2.clone(), self.curr) {
            (_, _, 0) if !self.v1.is_empty() => self.v1.pop_front().unwrap(),
            (_, _, 1) if !self.v2.is_empty() => self.v2.pop_front().unwrap(),
            (_, _, 0) if !self.v2.is_empty() => self.v2.pop_front().unwrap(),
            (_, _, 1) if !self.v1.is_empty() => self.v1.pop_front().unwrap(),
            _ => -1,
        };
        self.curr = (self.curr + 1) % 2;
        ans
    }

    fn has_next(&self) -> bool {
        !self.v1.is_empty() || !self.v2.is_empty()
    }
}

fn main() {
    let mut obj = ZigzagIterator::new(vec![1, 2], vec![3, 4, 5, 6]);
    let ret_1: i32 = obj.next();
    let ret_2: bool = obj.has_next();
    println!("ret_1 = {}, ret_2 = {}", ret_1, ret_2);
    let ret_1: i32 = obj.next();
    let ret_2: bool = obj.has_next();
    println!("ret_1 = {}, ret_2 = {}", ret_1, ret_2);
    let ret_1: i32 = obj.next();
    let ret_2: bool = obj.has_next();
    println!("ret_1 = {}, ret_2 = {}", ret_1, ret_2);
    let ret_1: i32 = obj.next();
    let ret_2: bool = obj.has_next();
    println!("ret_1 = {}, ret_2 = {}", ret_1, ret_2);
    let ret_1: i32 = obj.next();
    let ret_2: bool = obj.has_next();
    println!("ret_1 = {}, ret_2 = {}", ret_1, ret_2);
    let ret_1: i32 = obj.next();
    let ret_2: bool = obj.has_next();
    println!("ret_1 = {}, ret_2 = {}", ret_1, ret_2);
    let ret_1: i32 = obj.next();
    let ret_2: bool = obj.has_next();
    println!("ret_1 = {}, ret_2 = {}", ret_1, ret_2);
    let ret_1: i32 = obj.next();
    let ret_2: bool = obj.has_next();
    println!("ret_1 = {}, ret_2 = {}", ret_1, ret_2);
}

use std::collections::VecDeque;

#[derive(Default)]
struct MyStack {
    queue1: VecDeque<i32>,
    queue2: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, x: i32) {
        self.queue1.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        while self.queue1.len() > 1 {
            self.queue2.push_back(self.queue1.pop_front().unwrap());
        }
        let res = self.queue1.pop_front().unwrap();
        while !self.queue2.is_empty() {
            self.queue1.push_back(self.queue2.pop_front().unwrap());
        }
        res
    }

    fn top(&self) -> i32 {
        *self.queue1.back().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue1.is_empty()
    }
}

fn main() {
    let mut stack = MyStack::new();
    stack.push(1);
    stack.push(2);
    println!("{}", stack.top());
    println!("{}", stack.pop());
    println!("{}", stack.empty());
}

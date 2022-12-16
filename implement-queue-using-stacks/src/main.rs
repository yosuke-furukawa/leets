#[derive(Debug, Default)]
struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, x: i32) {
        while let Some(y) = self.stack2.pop() {
            self.stack1.push(y);
        }
        self.stack1.push(x);
        while let Some(y) = self.stack1.pop() {
            self.stack2.push(y);
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack2.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        *self.stack2.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack2.is_empty()
    }
}

fn main() {
    let mut q = MyQueue::new();
    q.push(1);
    q.push(2);
    println!("{:?}", q);
    println!("{}", q.peek());
    println!("{}", q.pop());
    println!("{}", q.empty());
}

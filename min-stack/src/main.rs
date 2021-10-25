#[derive(Default)]
struct MinStack {
    data: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack::default()
    }

    fn push(&mut self, val: i32) {
        self.data.push(val);
        if let Some(&last) = self.min_stack.last() {
            let min = last.min(val);
            self.min_stack.push(min);
            return;
        }
        self.min_stack.push(val);
    }

    fn pop(&mut self) {
        self.min_stack.pop();
        self.data.pop();
    }

    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

fn main() {
    let mut stack = MinStack::new();
    stack.push(-2);
    stack.push(0);
    stack.push(-3);
    println!("{}", stack.get_min());
    stack.pop();
    println!("{}", stack.top());
    println!("{}", stack.get_min());
}

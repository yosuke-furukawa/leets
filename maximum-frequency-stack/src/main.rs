use std::collections::HashMap;

#[derive(Default)]
struct FreqStack {
    freq_map: HashMap<i32, i32>,
    set_map: HashMap<i32, Vec<i32>>,
    max_freq: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        FreqStack::default()
    }

    fn push(&mut self, x: i32) {
        *self.freq_map.entry(x).or_insert(0) += 1;
        let freqs = *self.freq_map.get(&x).unwrap();
        self.max_freq = self.max_freq.max(freqs);
        self.set_map.entry(freqs).or_insert_with(Vec::new).push(x);
    }

    fn pop(&mut self) -> i32 {
        let value = self.set_map.get_mut(&self.max_freq).unwrap();
        let top = value.pop().unwrap();
        *self.freq_map.entry(top).or_insert(0) -= 1;
        if value.is_empty() {
            self.max_freq -= 1;
        }
        top
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 */

fn main() {
    let mut stack = FreqStack::new();
    stack.push(5);
    stack.push(7);
    stack.push(5);
    stack.push(7);
    stack.push(4);
    stack.push(5);

    println!("{}", stack.pop());
    println!("{}", stack.pop());
    println!("{}", stack.pop());
    println!("{}", stack.pop());
}

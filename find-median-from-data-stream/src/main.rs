use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self::default()
    }

    fn add_num(&mut self, num: i32) {
        if self.max_heap.is_empty() || num < *self.max_heap.peek().unwrap() {
            self.max_heap.push(num);
        } else {
            self.min_heap.push(Reverse(num));
        }

        if self.max_heap.len() as i32 - self.min_heap.len() as i32 > 1 {
            self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
        } else if self.min_heap.len() as i32 - self.max_heap.len() as i32 > 1 {
            self.max_heap.push(self.min_heap.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        match (self.max_heap.len(), self.min_heap.len()) {
            (max, min) if max > min => *self.max_heap.peek().unwrap() as f64,
            (max, min) if max < min => self.min_heap.peek().unwrap().0 as f64,
            _ => (self.max_heap.peek().unwrap() + self.min_heap.peek().unwrap().0) as f64 / 2.0,
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

fn main() {
    let mut obj = MedianFinder::new();
    obj.add_num(1);
    obj.add_num(2);
    println!("{}", obj.find_median());
    obj.add_num(3);
    println!("{}", obj.find_median());
}

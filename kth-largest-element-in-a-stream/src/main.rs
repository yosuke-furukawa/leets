use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        for n in nums {
            heap.push(Reverse(n));
            if heap.len() > k {
                heap.pop();
            }
        }
        KthLargest { heap, k }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

fn main() {
    let mut obj = KthLargest::new(3, vec![4, 5, 8, 2]);
    println!("{}", obj.add(3));
    println!("{}", obj.add(5));
    println!("{}", obj.add(10));
    println!("{}", obj.add(9));
    println!("{}", obj.add(4));
}

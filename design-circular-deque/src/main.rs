struct MyCircularDeque {
    data: Vec<i32>,
    head: usize,
    tail: usize,
    size: usize,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            data: vec![0; k as usize],
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.head = (self.head + self.data.len() - 1) % self.data.len();
        self.data[self.head] = value;
        self.size += 1;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.data[self.tail] = value;
        self.tail = (self.tail + 1) % self.data.len();
        self.size += 1;
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head = (self.head + 1) % self.data.len();
        self.size -= 1;
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.tail = (self.tail + self.data.len() - 1) % self.data.len();
        self.size -= 1;
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.head]
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[(self.tail + self.data.len() - 1) % self.data.len()]
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.data.len()
    }
}

fn main() {
    let mut obj = MyCircularDeque::new(3);
    println!("{}", obj.insert_last(1));
    println!("{}", obj.insert_last(2));
    println!("{}", obj.insert_front(3));
    println!("{}", obj.insert_front(4));
    println!("{}", obj.get_rear());
    println!("{}", obj.is_full());
    println!("{}", obj.delete_last());
    println!("{}", obj.insert_front(4));
    println!("{}", obj.get_front());
    println!("{}", obj.delete_front());
    println!("{}", obj.get_front());
}

struct MyCircularQueue {
    list: Vec<i32>,
    limit: usize,
    count: usize,
    first: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        let limit = k as usize;
        Self {
            list: vec![-1; limit],
            limit,
            count: 0,
            first: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.list[(self.first + self.count) % self.limit] = value;
        self.count += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.first = (self.first + 1) % (self.limit);
        self.count -= 1;
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.list[self.first]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.list[(self.first + self.count - 1) % self.limit]
        }
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn is_full(&self) -> bool {
        self.count == self.limit
    }
}

fn main() {
    let mut obj = MyCircularQueue::new(3);
    let ret_1: bool = obj.en_queue(10);
    let ret_2: bool = obj.de_queue();
    let ret_3: i32 = obj.front();
    let ret_4: i32 = obj.rear();
    let ret_5: bool = obj.is_empty();
    let ret_6: bool = obj.is_full();
    println!(
        "{} {} {} {} {} {}",
        ret_1, ret_2, ret_3, ret_4, ret_5, ret_6
    );
}

use std::collections::LinkedList;

struct MyHashMap {
    data: Vec<LinkedList<(i32, i32)>>,
    capacity: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    /** Initialize your data structure here. */
    fn new() -> Self {
        let capacity: i32 = 987;
        Self {
            data: vec![LinkedList::new(); capacity as usize],
            capacity,
        }
    }

    /** value will always be non-negative. */
    fn put(&mut self, key: i32, value: i32) {
        let hash = key % self.capacity;
        if self.data[hash as usize].is_empty() {
            self.data[hash as usize].push_back((key, value));
        } else {
            let mut found = false;
            for v in self.data[hash as usize].iter_mut() {
                if v.0 == key {
                    *v = (key, value);
                    found = true;
                    break;
                }
            }
            if !found {
                self.data[hash as usize].push_back((key, value));
            }
        }
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&self, key: i32) -> i32 {
        let hash = key % self.capacity;
        for v in self.data[hash as usize].iter() {
            if v.0 == key {
                return v.1;
            }
        }
        -1
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: i32) {
        let hash = key % self.capacity;
        self.data[hash as usize] = self.data[hash as usize]
            .iter()
            .filter(|&v| v.0 != key)
            .cloned()
            .collect();
    }
}

fn main() {
    println!("Hello, world!");
}

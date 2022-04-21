use std::collections::VecDeque;

const CAPACITY: i32 = 987;

#[derive(Debug, PartialEq, Eq)]
struct MyHashSet {
    data: Vec<Option<VecDeque<i32>>>,
    cap: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        Self {
            data: vec![None; CAPACITY as usize],
            cap: CAPACITY,
        }
    }

    fn add(&mut self, key: i32) {
        let hash = (key % self.cap) as usize;
        if let Some(list) = self.data[hash].as_mut() {
            if !list.contains(&key) {
                list.push_back(key);
            }
        } else {
            self.data[hash] = Some(VecDeque::from(vec![key]));
        }
    }

    fn remove(&mut self, key: i32) {
        let hash = (key % self.cap) as usize;
        if let Some(list) = self.data[hash].as_mut() {
            if let Some(index) = list.iter().position(|&x| x == key) {
                list.remove(index);
            }
        }
    }

    fn contains(&self, key: i32) -> bool {
        let hash = (key % self.cap) as usize;
        if let Some(list) = self.data[hash].as_ref() {
            return list.contains(&key);
        }
        false
    }
}

fn main() {
    let mut my_hash_set = MyHashSet::new();
    my_hash_set.add(1);
    my_hash_set.add(2);
    println!("{:?}", my_hash_set.contains(1));
    println!("{:?}", my_hash_set.contains(3));
    my_hash_set.add(2);
    println!("{:?}", my_hash_set.contains(2));
    my_hash_set.remove(2);
    println!("{:?}", my_hash_set.contains(2));
}

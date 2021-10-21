use rand::seq::SliceRandom;
use std::collections::HashMap;

struct RandomizedSet {
    data: HashMap<i32, usize>,
    keys: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            data: HashMap::new(),
            keys: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.data.contains_key(&val) {
            return false;
        }
        self.keys.push(val);
        let len = self.keys.len();
        self.data.insert(val, len - 1);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.data.contains_key(&val) {
            return false;
        }
        if let Some(&index) = self.data.get(&val) {
            let len = self.keys.len();
            self.keys.swap(index, len - 1);
            self.keys.pop();
            if index != len - 1 {
                let data = self.keys[index];
                self.data.insert(data, index);
            }
            self.data.remove(&val);
        }
        true
    }

    fn get_random(&self) -> i32 {
        *self.keys.choose(&mut rand::thread_rng()).unwrap()
    }
}

fn main() {
    let mut random_set = RandomizedSet::new();
    println!("{}", random_set.insert(1));
    println!("{}", random_set.insert(1));
    println!("{}", random_set.remove(1));
    println!("{}", random_set.insert(1));
    println!("{}", random_set.insert(2));
    println!("{}", random_set.insert(3));
    println!("{}", random_set.get_random());
}

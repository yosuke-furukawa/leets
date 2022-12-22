use std::collections::HashMap;

struct LFUCache {
    capacity: i32,
    min_freq: i32,
    freqs: HashMap<i32, Vec<(i32, i32)>>,
    cache: HashMap<i32, (i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            min_freq: 0,
            freqs: HashMap::new(),
            cache: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some((value, freq)) = self.cache.get_mut(&key) {
            self.freqs.get_mut(freq).unwrap().retain(|(k, _)| *k != key);
            if self.freqs.get(freq).unwrap().is_empty() {
                self.freqs.remove(freq);
                if self.min_freq == *freq {
                    self.min_freq += 1;
                }
            }
            *freq += 1;
            self.freqs.entry(*freq).or_default().push((key, *value));
            *value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        if let Some((v, freq)) = self.cache.get_mut(&key) {
            self.freqs.get_mut(freq).unwrap().retain(|(k, _)| *k != key);
            if self.freqs.get(freq).unwrap().is_empty() {
                self.freqs.remove(freq);
                if self.min_freq == *freq {
                    self.min_freq += 1;
                }
            }
            *freq += 1;
            self.freqs.entry(*freq).or_default().push((key, *v));
            *v = value;
        } else {
            if self.cache.len() == self.capacity as usize {
                let (k, _) = self.freqs.get_mut(&self.min_freq).unwrap().remove(0);
                self.cache.remove(&k);
            }
            self.min_freq = 1;
            self.freqs.entry(1).or_default().push((key, value));
            self.cache.insert(key, (value, 1));
        }
    }
}

fn main() {
    let mut obj = LFUCache::new(2);
    obj.put(1, 1);
    obj.put(2, 2);
    println!("{}", obj.get(1));
    obj.put(3, 3);
    println!("{}", obj.get(2));
    println!("{}", obj.get(3));
    obj.put(4, 4);
    println!("{}", obj.get(1));
    println!("{}", obj.get(3));
    println!("{}", obj.get(4));
}

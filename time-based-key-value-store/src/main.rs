use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Default)]
struct TimeMap {
    map: HashMap<String, BTreeMap<i32, String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Default::default()
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.map
            .get(&key)
            .and_then(|m| m.range(..=timestamp).next_back())
            .map(|(_, v)| v.to_string())
            .unwrap_or("".to_string())
    }
}

fn main() {
    let mut time_map = TimeMap::new();
    time_map.set("foo".to_string(), "bar".to_string(), 1);
    assert_eq!(time_map.get("foo".to_string(), 1), "bar".to_string());
    assert_eq!(time_map.get("foo".to_string(), 3), "bar".to_string());
    time_map.set("foo".to_string(), "bar2".to_string(), 4);
    assert_eq!(time_map.get("foo".to_string(), 4), "bar2".to_string());
    assert_eq!(time_map.get("foo".to_string(), 5), "bar2".to_string());
}

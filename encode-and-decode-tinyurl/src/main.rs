use rand::distributions::Alphanumeric;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;

const PREFIX: &str = "http://tinyurl.com/";

#[derive(Default)]
struct Codec {
    url_map: RefCell<HashMap<String, String>>,
    key_map: RefCell<HashMap<String, String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self::default()
    }

    // Encodes a URL to a shortened URL.
    fn encode(&self, long_url: String) -> String {
        if let Some(url) = self.url_map.borrow().get(&long_url) {
            return url.to_owned();
        }

        let key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        let tinyurl = PREFIX.to_string() + key.as_str();

        (*self.url_map.borrow_mut())
            .entry(long_url.clone())
            .or_insert_with(|| tinyurl.clone());
        (*self.key_map.borrow_mut())
            .entry(tinyurl.clone())
            .or_insert_with(|| long_url);
        tinyurl
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, short_url: String) -> String {
        self.key_map
            .borrow()
            .get(&short_url)
            .unwrap_or(&String::new())
            .clone()
    }
}

fn main() {
    let codec = Codec::new();
    let tinyurl = codec.encode("example.com".to_string());
    println!("{}", tinyurl);
    let longurl = codec.decode(tinyurl);
    println!("{}", longurl);
}

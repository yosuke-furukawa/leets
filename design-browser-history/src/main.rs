struct BrowserHistory {
    homepage: String,
    history: Vec<String>,
    forwards: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory {
            homepage,
            history: vec![],
            forwards: vec![],
        }
    }

    fn visit(&mut self, url: String) {
        self.history.push(url);
        self.forwards.clear();
    }

    fn back(&mut self, steps: i32) -> String {
        for _ in 0..steps as usize {
            if let Some(page) = self.history.pop() {
                self.forwards.push(page);
            } else {
                break;
            }
        }
        self.history.last().unwrap_or(&self.homepage).clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        for _ in 0..steps as usize {
            if let Some(page) = self.forwards.pop() {
                self.history.push(page);
            } else {
                break;
            }
        }
        self.history.last().unwrap_or(&self.homepage).clone()
    }
}

fn main() {
    let mut obj = BrowserHistory::new("leetcode.com".to_string());
    obj.visit("google.com".to_string());
    obj.visit("facebook.com".to_string());
    obj.visit("youtube.com".to_string());
    println!("{:?}", obj.back(1));
    println!("{:?}", obj.back(1));
    println!("{:?}", obj.forward(1));
    obj.visit("linkedin.com".to_string());
    println!("{:?}", obj.forward(2));
    println!("{:?}", obj.back(2));
    println!("{:?}", obj.back(7));
    let mut obj2 = BrowserHistory::new("zav.com".to_string());
    obj2.visit("kni.com".to_string());
    println!("{:?}", obj2.back(7));
    println!("{:?}", obj2.back(7));
    println!("{:?}", obj2.history);
    println!("{:?}", obj2.forwards);
    println!("{:?}", obj2.forward(5));
    println!("{:?}", obj2.forward(1));
    println!("{:?}", obj2.history);
    println!("{:?}", obj2.forwards);
    obj2.visit("pwrrbnw.com".to_string());
    obj2.visit("mosohif.com".to_string());
    println!("{:?}", obj2.history);
    println!("{:?}", obj2.forwards);
    println!("{:?}", obj2.back(9));
}

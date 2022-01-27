#[derive(Clone, Debug)]
struct Trie {
    children: Vec<Option<Box<Trie>>>,
    value: Option<i32>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: vec![None; 2],
            value: None,
        }
    }
    fn put(&mut self, bits: String, value: i32) {
        let mut node = self;
        for b in bits.chars() {
            let idx = b as usize - '0' as usize;
            if node.children[idx].is_none() {
                node.children[idx] = Some(Box::new(Trie::new()));
            }
            node = node.children[idx].as_mut().unwrap();
        }
        node.value = Some(value);
    }
    fn xor(&self, bits: String) -> i32 {
        let mut xor = 0;
        let mut node = self;
        for (i, b) in bits.chars().enumerate() {
            let idx = b as usize - '0' as usize;
            if node.children[1 - idx].is_some() {
                xor += 2_i32.pow(bits.len() as u32 - i as u32 - 1);
                node = node.children[1 - idx].as_ref().unwrap();
            } else {
                node = node.children[idx].as_ref().unwrap();
            }
        }
        xor
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        for n in nums.iter() {
            trie.put(format!("{:032b}", n), *n);
        }
        let mut max = 0;
        for n in nums.iter() {
            max = max.max(trie.xor(format!("{:032b}", n)));
        }
        max
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]));
    println!(
        "{}",
        Solution::find_maximum_xor(vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70])
    );
}

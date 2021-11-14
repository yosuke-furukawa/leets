struct CombinationIterator {
    combinations: Vec<String>,
}

fn next_combination(sub: i32) -> i32 {
    let x = sub & -sub;
    let y = sub + x;
    (((sub & !y) / x) >> 1) | y
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        let mut bit = (1 << combination_length) - 1;
        let mut combinations = vec![];
        while bit < (1 << characters.len() as i32) {
            let mut str = String::new();
            for (i, c) in characters.chars().enumerate() {
                if (bit & (1 << i as i32)) != 0 {
                    str += &c.to_string();
                }
            }
            combinations.push(str);
            bit = next_combination(bit);
        }
        combinations.sort_unstable();
        Self {
            combinations: combinations.into_iter().rev().collect(),
        }
    }

    fn next(&mut self) -> String {
        self.combinations.pop().unwrap()
    }

    fn has_next(&self) -> bool {
        !self.combinations.is_empty()
    }
}

fn main() {
    let mut itr = CombinationIterator::new("abc".to_string(), 2);
    println!("{}", itr.next());
    println!("{}", itr.has_next());
    println!("{}", itr.next());
    println!("{}", itr.has_next());
    println!("{}", itr.next());
    println!("{}", itr.has_next());
}

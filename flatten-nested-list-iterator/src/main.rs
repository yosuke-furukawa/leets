struct NestedIterator {
    cursor: usize,
    list: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {

    fn flat(nested_list: Vec<NestedInteger>, acc: &mut Vec<i32>) {
        for item in nested_list.into_iter() {
            match item {
                NestedInteger::Int(x) => {
                    acc.push(x);
                }
                NestedInteger::List(x) => {
                    Self::flat(x, acc);
                }
            }
        }
    }

    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut acc = vec![];
        Self::flat(nestedList, &mut acc);
        Self{ cursor: 0, list: acc }
    }
    
    fn next(&mut self) -> i32 {
        if let Some(x) = self.list.get(self.cursor) {
            self.cursor += 1;
            *x
        } else {
            0
        }
    }
    
    fn has_next(&self) -> bool {
        self.cursor < self.list.len()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

fn main() {
    println!("Hello, world!");
}

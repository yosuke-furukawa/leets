impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack = vec![];
        let mut h = &head;
        while let Some(n) = h {
            stack.push(n.val);
            h = &n.next;
        }
        let mut current = None;
        for &v in stack.iter() {
            let mut node = ListNode::new(v);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
    }
}

struct Solution;

pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

macro_rules! linked {
    ($($e:expr),*) => {to_list(vec![$($e.to_owned()), *])};
    ($($e:expr,)*) => {to_list(vec![$($e.to_owned()), *])};
}

fn main() {
    println!("{:?}", Solution::reverse_list(linked![1, 2, 3, 4, 5]));
}

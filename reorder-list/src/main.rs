use std::collections::VecDeque;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut queue = VecDeque::new();
        let mut h = head.take();
        while let Some(mut n) = h {
            h = n.next.take();
            queue.push_back(Some(n));
        }
        let mut stack = vec![];
        let mut is_front = true;
        while !queue.is_empty() {
            if is_front {
                stack.push(queue.pop_front().unwrap());
            } else {
                stack.push(queue.pop_back().unwrap())
            }
            is_front = !is_front;
        }
        let mut prev = None;
        while let Some(last) = stack.pop() {
            let mut node = last.unwrap();
            node.next = prev;
            prev = Some(node);
        }
        *head = prev
    }
}

struct Solution;
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

pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

macro_rules! linked {
    ($($e:expr),*) => {to_list(vec![$($e.to_owned()), *])};
    ($($e:expr,)*) => {to_list(vec![$($e.to_owned()), *])};
}

fn main() {
    let mut list = linked![1, 2, 3, 4];
    Solution::reorder_list(&mut list);
    println!("{:?}", list);
}

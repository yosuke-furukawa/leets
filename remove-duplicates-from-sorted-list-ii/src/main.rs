impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack = vec![];
        let mut head = head;
        while let Some(node) = head {
            let val = node.val;
            match stack.last() {
                Some(&(pval, count)) if pval == val => {
                    stack.pop();
                    stack.push((val, count + 1));
                }
                _ => {
                    stack.push((val, 1));
                }
            }
            head = node.next;
        }
        to_list(
            stack
                .iter()
                .filter(|(_, count)| *count == 1)
                .map(|(val, _)| *val)
                .collect(),
        )
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

macro_rules! linked {
    ($($e:expr),*) => {to_list(vec![$($e.to_owned()), *])};
    ($($e:expr,)*) => {to_list(vec![$($e.to_owned()), *])};
}

fn main() {
    println!(
        "{:?}",
        Solution::delete_duplicates(linked![1, 2, 3, 3, 4, 4, 5])
    );
}

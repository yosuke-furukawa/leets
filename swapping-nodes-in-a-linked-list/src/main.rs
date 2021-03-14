impl Solution {
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut curr = head;
        let mut array = vec![];
        while let Some(mut inner) = curr {
            array.push(inner.val);
            curr = inner.next.take();
        }

        let len = array.len();
        array.swap(k as usize - 1, len - (k as usize - 1) - 1);

        let mut current = None;
        for &v in array.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
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

// helper function for test
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

#[macro_export]
macro_rules! linked {
    ($($e:expr),*) => {to_list(vec![$($e.to_owned()), *])};
    ($($e:expr,)*) => {to_list(vec![$($e.to_owned()), *])};
}

fn main() {
    println!("{:?}", Solution::swap_nodes(linked![1, 2, 3, 4, 5], 2));
}

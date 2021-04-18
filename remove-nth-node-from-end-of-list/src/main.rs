impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode{val: -1, next: head});
        let mut fast = dummy.clone();
        let mut slow = dummy.as_mut();

        for _ in 0..n {
            fast = fast.next.unwrap();
        }

        while fast.next.is_some() {
            slow = slow.next.as_mut().unwrap();
            fast = fast.next.unwrap();
        }
        let next = slow.next.as_mut().unwrap();
        slow.next = next.next.clone();
        dummy.next
    }
}

struct Solution;

#[derive(PartialEq, Eq, Debug, Clone)]
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
    println!("{:?}", Solution::remove_nth_from_end(linked![1,2,3,4,5], 2));
}

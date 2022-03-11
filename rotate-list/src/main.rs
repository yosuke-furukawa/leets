impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        if head.is_none() {
            return head;
        }
        let mut cnt = 0;
        let mut th = &head;
        while th.is_some() {
            th = &th.as_ref().unwrap().next;
            cnt += 1;
        }
        let k = k % cnt;
        if k < 1 {
            return head;
        }
        let p = cnt - k;
        let mut th = &mut head;
        for _ in 0..p-1 {
            th = &mut th.as_mut().unwrap().next;
        }
        let mut new_head = th.as_mut().unwrap().next.take();
        let mut ref_new = &mut new_head;
        while ref_new.is_some() && ref_new.as_ref().unwrap().next.is_some() {
            ref_new = &mut ref_new.as_mut().unwrap().next;
        }
        ref_new.as_mut().map(|x| x.next = head);

        new_head
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
    println!("{:?}", Solution::rotate_right(linked![1,2,3,4,5], 2));
}

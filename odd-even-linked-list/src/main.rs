impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd_head = ListNode::new(-1);
        let mut even_head = ListNode::new(-1);
        let mut odd_cur = &mut odd_head;
        let mut even_cur = &mut even_head;
        let mut is_even = false;
        while let Some(mut node) = head {
            head = node.next.take();
            if is_even {
                even_cur.next = Some(node);
                even_cur = even_cur.next.as_mut().unwrap();
            } else {
                odd_cur.next = Some(node);
                odd_cur = odd_cur.next.as_mut().unwrap();
            }
            is_even = !is_even;
        }

        odd_cur.next = even_head.next;
        odd_head.next
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
    println!("{:?}", Solution::odd_even_list(linked![1, 2, 3, 4, 5]));
}

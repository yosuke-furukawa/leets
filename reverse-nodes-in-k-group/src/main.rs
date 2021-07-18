impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut result = vec![];
        let mut h = head;
        while h.is_some() {
            let mut stack = vec![];
            let mut n = h;
            for _ in 0..k {
                if n.is_none() {
                    break;
                }
                let nval = n.unwrap();
                stack.push(nval.val);
                n = nval.next;
            }
            if stack.len() == k as usize {
                for n in stack.iter().rev() {
                    result.push(*n);
                }
            } else {
                for n in stack.iter() {
                    result.push(*n);
                }
            }
            h = n;
        }

        to_list(result)
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
    println!("{:?}", Solution::reverse_k_group(linked![1, 2, 3, 4, 5], 2));
}

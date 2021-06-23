impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        head.as_ref()?;
        let left = left as usize;
        let right = right as usize;

        let mut cur = &head;
        let mut arr = vec![];
        while let Some(c) = cur {
            arr.push(c.val);
            cur = &c.next;
        }

        arr[left - 1..right].reverse();

        to_list(arr.into_iter().collect())
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
        Solution::reverse_between(linked![1, 2, 3, 4, 5], 2, 4)
    );
    println!("{:?}", Solution::reverse_between(linked![3, 5], 1, 2));
}

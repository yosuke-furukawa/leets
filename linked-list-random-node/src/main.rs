use rand::{rngs::ThreadRng, Rng};
struct Solution {
    head: Option<Box<ListNode>>,
    len: usize,
    rng: ThreadRng,
}

impl Solution {
    pub fn new(head: Option<Box<ListNode>>) -> Self {
        Self {
            len: (0..)
                .scan(&head, |node, _| {
                    node.as_deref().map(|next| *node = &next.next)
                })
                .fuse()
                .count(),
            rng: ThreadRng::default(),
            head,
        }
    }

    pub fn get_random(&mut self) -> i32 {
        (0..self.rng.gen_range(0, self.len))
            .fold(self.head.as_deref().unwrap(), |ptr, _| {
                ptr.next.as_deref().unwrap()
            })
            .val
    }
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
    let mut l = Solution::new(linked![1, 2, 3]);

    println!("{}", l.get_random());
}

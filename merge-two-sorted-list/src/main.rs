impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = &list1;
        let mut list2 = &list2;
        let mut array = vec![];
        while list1.is_some() || list2.is_some() {
            match (list1, list2) {
                (Some(l1), Some(l2)) => {
                    if l1.val < l2.val {
                        array.push(l1.val);
                        list1 = &l1.next;
                    } else {
                        array.push(l2.val);
                        list2 = &l2.next;
                    }
                }
                (Some(l1), None) => {
                    array.push(l1.val);
                    list1 = &l1.next;
                }
                (None, Some(l2)) => {
                    array.push(l2.val);
                    list2 = &l2.next;
                }
                (None, None) => {
                    break;
                }
            }
        }
        to_list(array)
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
        Solution::merge_two_lists(linked![1, 2, 4], linked![1, 3, 4])
    );
}

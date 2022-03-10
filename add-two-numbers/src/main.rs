impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut array = vec![];
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry > 0 {
            match (l1, l2) {
                (Some(l_1), Some(l_2)) => {
                    let sum = l_1.val + l_2.val + carry;
                    array.push(sum % 10);
                    carry = sum / 10;
                    l1 = &l_1.next;
                    l2 = &l_2.next;
                }
                (Some(l_1), None) => {
                    let sum = l_1.val + carry;
                    array.push(sum % 10);
                    carry = sum / 10;
                    l1 = &l_1.next;
                }
                (None, Some(l_2)) => {
                    let sum = l_2.val + carry;
                    array.push(sum % 10);
                    carry = sum / 10;
                    l2 = &l_2.next;
                }
                (None, None) => {
                    if carry > 0 {
                        array.push(carry);
                    }
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
        Solution::add_two_numbers(linked![1, 1, 2], linked![1, 1, 2])
    );
    println!(
        "{:?}",
        Solution::add_two_numbers(linked![9, 9, 9, 9, 9, 9, 9], linked![9, 9, 9, 9])
    );
}

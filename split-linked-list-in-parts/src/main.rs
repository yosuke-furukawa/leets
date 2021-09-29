impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut res = vec![];
        let mut array = vec![];
        let mut pointer = head;
        while let Some(node) = pointer {
            array.push(node.val);
            pointer = node.next;
        }
        let mut i = 0;
        let n = array.len();
        while i < n {
            let mut j = 1;
            let mut temp = vec![];
            while j <= n / k as usize {
                temp.push(array[i + j - 1]);
                j += 1;
            }

            if res.len() < n % k as usize {
                temp.push(array[i + j - 1]);
                j += 1;
            }

            res.push(to_list(temp));
            i += j - 1;
        }
        while res.len() < k as usize {
            res.push(None);
        }
        res
    }
}

fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
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
    println!("{:?}", Solution::split_list_to_parts(linked![1, 2, 3], 5));
    println!(
        "{:?}",
        Solution::split_list_to_parts(linked![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3)
    );
}

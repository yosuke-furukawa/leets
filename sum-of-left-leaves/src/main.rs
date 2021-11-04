use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn helper(node: Option<&Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(n) = node {
            let node = n.borrow();
            let left = node.left.as_ref();
            if left.is_some()
                && left.unwrap().borrow().left.is_none()
                && left.unwrap().borrow().right.is_none()
            {
                if let Some(l) = left {
                    *sum += l.borrow().val;
                }
            }
            Self::helper(node.left.as_ref(), sum);
            Self::helper(node.right.as_ref(), sum);
        }
    }
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::helper(root.as_ref(), &mut sum);
        sum
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());

    for children in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(v) = children[0] {
            parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }
        if children.len() > 1 {
            if let Some(v) = children[1] {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    head
}

#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}

struct Solution;

fn main() {
    println!(
        "{}",
        Solution::sum_of_left_leaves(tree![3, 9, 20, null, null, 15, 7])
    );
    println!("{}", Solution::sum_of_left_leaves(tree![1]));
}

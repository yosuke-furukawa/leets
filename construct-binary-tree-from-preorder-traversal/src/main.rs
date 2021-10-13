use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn helper(node: &mut Rc<RefCell<TreeNode>>, value: i32) {
        let mut node = node.borrow_mut();
        if node.val >= value {
            if let Some(left) = &mut node.left {
                Self::helper(left, value);
            } else {
                node.left = Some(Rc::new(RefCell::new(TreeNode::new(value))));
            }
        } else if let Some(right) = &mut node.right {
            Self::helper(right, value);
        } else {
            node.right = Some(Rc::new(RefCell::new(TreeNode::new(value))));
        }
    }
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut head = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
        for n in preorder.into_iter().skip(1) {
            Self::helper(&mut head, n);
        }

        Some(head)
    }
}

struct Solution;

#[derive(Debug, PartialEq, Eq)]
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

fn main() {
    println!(
        "{:?}",
        Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12])
    );
}

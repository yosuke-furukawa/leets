use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn traverse(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut String) {
        if let Some(n) = node {
            let b = n.borrow();
            result.push_str(&b.val.to_string());
            if b.left.is_none() && b.right.is_none() {
                return;
            }
            if b.left.is_some() {
                result.push('(');
                Self::traverse(b.left.as_ref(), result);
                result.push(')');
            }
            if b.left.is_none() {
                result.push('(');
                result.push(')');
            }
            if b.right.is_some() {
                result.push('(');
                Self::traverse(b.right.as_ref(), result);
                result.push(')');
            }
        }
    }
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = "".to_string();
        Self::traverse(root.as_ref(), &mut result);
        result
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

fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
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

fn main() {
    println!("{}", Solution::tree2str(tree![1, 2, 3, 4]));
    println!("{}", Solution::tree2str(tree![1, 2, 3, null, 4]));
}

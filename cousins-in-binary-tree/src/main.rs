use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn get_depth(
        node: Option<&Rc<RefCell<TreeNode>>>,
        target: i32,
        parent: Option<&Rc<RefCell<TreeNode>>>,
        depth: i32,
    ) -> (i32, i32) {
        if let Some(n) = node {
            let n = n.borrow();
            if let Some(p) = parent {
                if n.val == target {
                    return (depth, p.borrow().val);
                }
            }
            let left = Self::get_depth(n.left.as_ref(), target, node, depth + 1);
            if left.0 >= 0 {
                return left;
            }
            let right = Self::get_depth(n.right.as_ref(), target, node, depth + 1);
            if right.0 >= 0 {
                return right;
            }
        }
        (-1, -1)
    }
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let xn = Self::get_depth(root.as_ref(), x, None, 0);
        let yn = Self::get_depth(root.as_ref(), y, None, 0);
        xn.0 >= 0 && yn.0 >= 0 && xn.1 != yn.1 && xn.0 == yn.0
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
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let mut queue = std::collections::VecDeque::new();
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
    println!(
        "{}",
        Solution::is_cousins(tree![1, 2, 3, null, 4, null, 5], 5, 4)
    );
    println!("{}", Solution::is_cousins(tree![1, 2, 3, 4], 4, 3));
    println!("{}", Solution::is_cousins(tree![1, 2, 3, null, 4], 2, 3));
    println!(
        "{}",
        Solution::is_cousins(tree![1, 2, 3, null, null, 4, 5], 4, 5)
    );
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn helper(
        node: Option<&Rc<RefCell<TreeNode>>>,
        depth: i32,
        is_left: bool,
        max_depth: &mut i32,
        most_left: &mut i32,
    ) {
        if let Some(node) = node {
            let n = node.borrow();
            if is_left && *max_depth < depth {
                *max_depth = depth;
                *most_left = n.val;
            }
            Self::helper(n.left.as_ref(), depth + 1, true, max_depth, most_left);
            Self::helper(
                n.right.as_ref(),
                depth + 1,
                n.left.is_none(),
                max_depth,
                most_left,
            );
        }
    }
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        let mut most_left = 0;
        Self::helper(root.as_ref(), 1, true, &mut max_depth, &mut most_left);
        most_left
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
    println!("{}", Solution::find_bottom_left_value(tree![2, 1, 3]));
    println!(
        "{}",
        Solution::find_bottom_left_value(tree![1, 2, 3, 4, null, 5, 6, null, null, 7])
    );
    println!("{}", Solution::find_bottom_left_value(tree![0, null, -1]));
}

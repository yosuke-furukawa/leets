use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, lower: Option<i32>, upper: Option<i32>) -> bool {
        if let Some(n) = node {
            let n = n.borrow();
            if lower.is_some() && lower.unwrap() >= n.val {
                return false;
            }
            if upper.is_some() && upper.unwrap() <= n.val {
                return false;
            }
            Self::dfs(n.left.as_ref(), lower, Some(n.val))
                && Self::dfs(n.right.as_ref(), Some(n.val), upper)
        } else {
            true
        }
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(root.as_ref(), None, None)
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
    println!("{}", Solution::is_valid_bst(tree![2, 1, 3]));
    println!(
        "{}",
        Solution::is_valid_bst(tree![5, 1, 4, null, null, 3, 6])
    );
    println!("{}", Solution::is_valid_bst(tree![2147483647]));
}

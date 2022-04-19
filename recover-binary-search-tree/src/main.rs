use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn inorder(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(n) = root {
            let n = n.borrow();
            Self::inorder(n.left.as_ref(), res);
            res.push(n.val);
            Self::inorder(n.right.as_ref(), res);
        }
    }
    fn recover(root: &mut Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(n) = root {
            let mut n = n.borrow_mut();
            Self::recover(&mut n.left, res);
            if let Some(v) = res.pop() {
                if n.val != v {
                    n.val = v;
                }
            }
            Self::recover(&mut n.right, res);
        }
    }
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut vals = vec![];
        Self::inorder(root.as_ref(), &mut vals);
        vals.sort_unstable_by(|a, b| b.cmp(a));
        Self::recover(root, &mut vals);
    }
}

struct Solution;

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
    let mut tree = tree![1, 3, null, null, 2];
    Solution::recover_tree(&mut tree);
    println!("{:?}", tree);
}

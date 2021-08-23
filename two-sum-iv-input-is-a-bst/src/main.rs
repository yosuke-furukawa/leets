use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

impl Solution {
    fn update_and_check(
        node: Option<&Rc<RefCell<TreeNode>>>,
        k: i32,
        set: &mut HashSet<i32>,
    ) -> bool {
        if let Some(n) = node {
            let val = n.borrow().val;
            if set.contains(&(k - val)) {
                return true;
            }
            set.insert(val);
            if n.borrow().left.is_some() && Self::update_and_check(n.borrow().left.as_ref(), k, set) {
                return true;
            }

            if n.borrow().right.is_some() && Self::update_and_check(n.borrow().right.as_ref(), k, set) {
                return true;
            }
        }
        false
    }
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut set = HashSet::new();
        Self::update_and_check(root.as_ref(), k, &mut set)
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
    println!(
        "{}",
        Solution::find_target(tree![5, 3, 6, 2, 4, null, 7], 9)
    );
    println!(
        "{}",
        Solution::find_target(tree![5, 3, 6, 2, 4, null, 7], 28)
    );
    println!("{}", Solution::find_target(tree![2, 1, 3], 4));
    println!("{}", Solution::find_target(tree![2, 1, 3], 1));
    println!("{}", Solution::find_target(tree![2, 1, 3], 3));
    println!("{}", Solution::find_target(tree![2, 0, 3, -4, 1], -1));
    println!(
        "{}",
        Solution::find_target(tree![4, 2, null, -3, null, null, -1, null, 0], -3)
    );
}

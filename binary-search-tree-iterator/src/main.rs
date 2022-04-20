use std::cell::RefCell;
use std::rc::Rc;

struct BSTIterator {
    stack: Vec<i32>,
}

fn traverse(node: Option<&Rc<RefCell<TreeNode>>>, stack: &mut Vec<i32>) {
    if let Some(n) = node {
        let n = n.borrow();
        traverse(n.right.as_ref(), stack);
        stack.push(n.val);
        traverse(n.left.as_ref(), stack);
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = vec![];
        traverse(root.as_ref(), &mut stack);
        Self { stack }
    }

    fn next(&mut self) -> i32 {
        self.stack.pop().unwrap()
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
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
    let mut bst_iterator = BSTIterator::new(tree![7, 3, 15, null, null, 9, 20]);
    println!("{}", bst_iterator.next());
    println!("{}", bst_iterator.next());
    println!("{}", bst_iterator.has_next());
    println!("{}", bst_iterator.next());
    println!("{}", bst_iterator.has_next());
    println!("{}", bst_iterator.next());
    println!("{}", bst_iterator.has_next());
    println!("{}", bst_iterator.next());
    println!("{}", bst_iterator.has_next());
}

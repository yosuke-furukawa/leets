use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::rc::Rc;

impl Solution {
    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        x: i32,
        y: i32,
        heap: &mut BinaryHeap<(i32, i32, i32)>,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            heap.push((x, y, -node.val));
            Self::dfs(&node.left, x - 1, y - 1, heap);
            Self::dfs(&node.right, x + 1, y - 1, heap);
        }
    }
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::<(i32, i32, i32)>::new();
        Self::dfs(&root, 0, 0, &mut heap);
        let mut ans = vec![];
        let mut x = std::i32::MIN;
        let mut v = vec![];
        while !heap.is_empty() {
            let h = heap.pop().unwrap();
            if h.0 != x {
                if !v.is_empty() {
                    ans.insert(0, v);
                }
                x = h.0;
                v = vec![];
            }
            v.push(-h.2);
        }
        if !v.is_empty() {
            ans.insert(0, v);
        }
        ans
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
        "{:?}",
        Solution::vertical_traversal(tree![3, 9, 20, null, null, 15, 7])
    );
}

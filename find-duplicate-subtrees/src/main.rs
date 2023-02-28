use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    fn helper(
        node: &Option<Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<String, i32>,
        res: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> String {
        if let Some(n) = node {
            let n = n.borrow();
            let left = Self::helper(&n.left, map, res);
            let right = Self::helper(&n.right, map, res);
            let s = format!("({}{},{})", left, n.val, right);
            if let Some(count) = map.get_mut(&s) {
                if *count == 1 {
                    res.push(node.clone());
                }
                *count += 1;
            } else {
                map.insert(s.clone(), 1);
            }
            s
        } else {
            String::from("null")
        }
    }
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut map = HashMap::new();
        let mut res = vec![];
        Self::helper(&root, &mut map, &mut res);
        res
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
        Solution::find_duplicate_subtrees(tree![1, 2, 3, 4, null, 2, 4, null, null, 4])
    );
}

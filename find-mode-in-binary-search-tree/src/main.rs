use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let mut counter = std::collections::HashMap::<i32, i32>::new();
                fn find(
                    n: Option<Rc<RefCell<TreeNode>>>,
                    m: &mut std::collections::HashMap<i32, i32>,
                ) {
                    match n {
                        Some(n) => {
                            (*m.entry(n.borrow().val).or_insert(0)) += 1;
                            find(n.borrow().left.clone(), m);
                            find(n.borrow().right.clone(), m);
                        }
                        None => (),
                    }
                }
                find(Some(root), &mut counter);
                let mut list = counter.iter().collect::<Vec<(&i32, &i32)>>();
                list.sort_unstable_by(|a, b| b.1.cmp(a.1));

                let (mut res, prev) = (vec![*list[0].0], list[0].1);
                for node in list.iter().skip(1) {
                    if prev != node.1 {
                        break;
                    }
                    res.push(*(node.0));
                }
                res
            }
        }
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
    println!("{:?}", Solution::find_mode(tree![1, null, 2, 2]));
}

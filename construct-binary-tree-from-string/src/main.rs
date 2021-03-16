use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn str2tree(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let chars: Vec<char> = s.chars().collect();
        let (res, _) = Self::helper(&chars);
        res
    }

    fn helper(chars: &[char]) -> (Option<Rc<RefCell<TreeNode>>>, &[char]) {
        if chars.len() == 0 {
            return (None, chars);
        }

        let (val, chars) = Self::parse_int(chars);
        let mut tree = TreeNode::new(val);
        if chars.len() == 0 {
            return (Some(Rc::new(RefCell::new(tree))), chars);
        }
        if chars[0] == ')' {
            return (Some(Rc::new(RefCell::new(tree))), &chars[1..]);
        }
        let (l_tree, chars) = Self::helper(&chars[1..]);
        tree.left = l_tree;
        if chars.len() == 0 || chars[0] == ')' {
            let chars = if chars.len() == 0 { chars } else { &chars[1..] };
            return (Some(Rc::new(RefCell::new(tree))), chars);
        }
        let (r_tree, chars) = Self::helper(&chars[1..]);
        tree.right = r_tree;

        let chars = if chars.len() == 0 { chars } else { &chars[1..] };
        return (Some(Rc::new(RefCell::new(tree))), chars);
    }

    fn parse_int(chars: &[char]) -> (i32, &[char]) {
        let is_minus = chars[0] == '-';
        let mut cur = if is_minus { 1 } else { 0 };
        let mut digits = vec![];
        while cur < chars.len() {
            if let Some(digit) = chars[cur].to_digit(10) {
                digits.push(digit);
                cur += 1;
            } else {
                break;
            }
        }
        let mut res = 0;
        for digit in digits {
            res *= 10;
            res += digit as i32;
        }
        if is_minus {
            res = -res;
        }
        (res, &chars[cur..])
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

fn main() {
    println!("Hello, world!");
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, d: i32) -> i32 {
        if root.is_none() {
            return d;
        }
        let r = root.clone().unwrap();
        let left = r.borrow().left.clone();
        let right = r.borrow().right.clone();
        d.max(dfs(left, d + 1)).max(dfs(right, d + 1))
    }
    dfs(root, 0)
}

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn h(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let r = root.unwrap();
        let left = r.borrow().left.clone();
        let right = r.borrow().right.clone();
        let hl = h(left);
        let hr = h(right);
        if hl < 0 || hr < 0 || (hl - hr).abs() > 1 {
            return -1;
        }
        hl.max(hr) + 1
    }

    if root.is_none() {
        return true;
    }
    h(root) > -1
}

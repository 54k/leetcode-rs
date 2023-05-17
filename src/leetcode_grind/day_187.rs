// https://leetcode.com/problems/binary-search-tree-iterator-ii/
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

struct BSTIterator {}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        todo!()
    }

    fn has_next(&self) -> bool {
        todo!()
    }

    fn next(&self) -> i32 {
        todo!()
    }

    fn has_prev(&self) -> bool {
        todo!()
    }

    fn prev(&self) -> i32 {
        todo!()
    }
}

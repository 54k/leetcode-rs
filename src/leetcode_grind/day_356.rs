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

use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/add-one-row-to-tree/description/
pub fn add_one_row(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
    depth: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    type RTN = Rc<RefCell<TreeNode>>;

    fn rec(node: Option<RTN>, curd: i32, val: i32, depth: i32) -> Option<RTN> {
        if node.is_none() {
            return node;
        }

        let n = node.clone().unwrap();
        let mut n = n.borrow_mut();

        if curd == depth - 1 {
            let mut new_n_left = TreeNode::new(val);
            new_n_left.left = n.left.clone();

            let mut new_n_right = TreeNode::new(val);
            new_n_right.right = n.right.clone();

            n.left = Some(Rc::new(RefCell::new(new_n_left)));
            n.right = Some(Rc::new(RefCell::new(new_n_right)));
            return node;
        }

        rec(n.left.clone(), curd + 1, val, depth);
        rec(n.right.clone(), curd + 1, val, depth);

        node
    }

    if depth == 1 {
        let mut new_n_left = TreeNode::new(val);
        new_n_left.left = root.clone();
        return Some(Rc::new(RefCell::new(new_n_left)));
    }

    rec(root, 1, val, depth)
}

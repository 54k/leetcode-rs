// https://leetcode.com/problems/balance-a-binary-search-tree/description/?envType=daily-question&envId=2024-06-26
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
pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn inorder(root: Node, v: &mut Vec<i32>) {
        if let Some(r) = root {
            let r = r.borrow();
            inorder(r.left.clone(), v);
            v.push(r.val);
            inorder(r.right.clone(), v);
        }
    }
    fn build(left: usize, right: usize, v: &Vec<i32>) -> Node {
        if left >= right {
            return None;
        }
        if left + 1 == right {
            return Some(Rc::new(RefCell::new(TreeNode {
                val: v[left],
                left: None,
                right: None,
            })));
        }
        let mid = (left + right) / 2;
        let ltree = build(left, mid, v);
        let rtree = build(mid + 1, right, v);
        Some(Rc::new(RefCell::new(TreeNode {
            val: v[mid],
            left: ltree,
            right: rtree,
        })))
    }

    let mut v = vec![];
    inorder(root.clone(), &mut v);
    build(0, v.len(), &v)
}

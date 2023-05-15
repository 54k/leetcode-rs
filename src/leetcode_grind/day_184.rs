use std::{cell::RefCell, rc::Rc};

// https://leetcode.com/problems/inorder-successor-in-bst/description/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn inorder_successor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn inorder(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
        ans: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(r) = root.clone() {
            let r = r.borrow();
            inorder(r.left.clone(), p.clone(), prev, ans);

            if *prev == p && ans.is_none() {
                *ans = root.clone();
                return;
            }
            *prev = root.clone();

            inorder(r.right.clone(), p.clone(), prev, ans);
        }
    }

    if p.as_ref()?.borrow().right.is_some() {
        let mut left_most = p.as_ref()?.borrow().right.clone()?;
        while left_most.clone().borrow().left.is_some() {
            left_most = left_most.clone().borrow().left.clone()?;
        }
        Some(left_most)
    } else {
        let mut ans = None;
        inorder(root, p, &mut None, &mut ans);
        ans
    }
}

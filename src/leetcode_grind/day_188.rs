use std::{cell::RefCell, rc::Rc};

// https://leetcode.com/problems/same-tree/description/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn check(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_some() && q.is_some() {
            p.as_ref().unwrap().borrow().val == q.as_ref().unwrap().borrow().val
        } else if p.is_none() && q.is_none() {
            true
        } else {
            false
        }
    }

    let mut ps = vec![];
    let mut qs = vec![];

    ps.push(p);
    qs.push(q);

    while ps.len() > 0 {
        let pe = ps.pop().unwrap();
        let qe = qs.pop().unwrap();

        if !check(pe.clone(), qe.clone()) {
            return false;
        }

        if pe.is_some() {
            ps.push(pe.as_ref().unwrap().borrow().right.clone());
            ps.push(pe.as_ref().unwrap().borrow().left.clone());
            
            qs.push(qe.as_ref().unwrap().borrow().right.clone());
            qs.push(qe.as_ref().unwrap().borrow().left.clone());
        }
    }

    true
}

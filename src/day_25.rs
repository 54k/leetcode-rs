#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    let mut res = 0;
    let mut stack = vec![];
    stack.push(root);

    while !stack.is_empty() {
        let root = stack.pop().unwrap();
        if let Some(r) = root {
            let x = r.borrow();
            if low <= x.val && x.val <= high {
                res += x.val;
            }
            if low < x.val {
                stack.push(x.left.clone());
            }
            if x.val < high {
                stack.push(x.right.clone());
            }
        }
    }
    res
}

#[allow(dead_code)]
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // lolololololol derp derp
    fn lol(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        p == q
    }
    fn dfs(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return if p.is_none() && q.is_none() {
            true
        } else if p.is_some() && q.is_some() {
            let p = p.unwrap();
            let q = q.unwrap();

            let p_borrowed = p.borrow();
            let q_borrowed = q.borrow();

            p_borrowed.val == q_borrowed.val
                && dfs(p_borrowed.left.clone(), q_borrowed.left.clone())
                && dfs(p_borrowed.right.clone(), q_borrowed.right.clone())
        } else {
            false
        };
    }
    dfs(p.clone(), q.clone()) && lol(p, q)
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/maximum-depth-of-binary-tree/
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::collections::*;
    let mut ans = 0;
    let mut q = VecDeque::new();
    q.push_back(root);
    while !q.is_empty() {
        let mut len = q.len();
        ans += 1;
        while len > 0 {
            len -= 1;
            if let Some(n) = q.pop_front().unwrap() {
                let n = n.borrow();
                if n.left.is_some() {
                    q.push_back(n.left.clone());
                }
                if n.right.is_some() {
                    q.push_back(n.right.clone());
                }
            }
        }
    }
    ans
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test229() {
        println!(
            "{}",
            max_depth(Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))))
        );
    }
}

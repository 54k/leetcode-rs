use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/count-univalue-subtrees/description
pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> bool {
        if root.is_none() {
            return true;
        }
        let r = root.as_ref().unwrap();
        let r = r.borrow();
        let left_uni = dfs(r.left.clone(), ans);
        let right_uni = dfs(r.right.clone(), ans);

        if left_uni && right_uni {
            if let Some(left_child) = r.left.clone() {
                if left_child.borrow().val != r.val {
                    return false;
                }
            }
            if let Some(right_child) = r.right.clone() {
                if right_child.borrow().val != r.val {
                    return false;
                }
            }
            *ans += 1;
            return true;
        }
        false
    }
    let mut ans = 0;
    dfs(root, &mut ans);
    ans
}

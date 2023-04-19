use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/description/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, go_left: bool, steps: i32, longest_path: &mut i32) {
        if let Some(r) = root {
            *longest_path = (*longest_path).max(steps);
            if go_left {
                dfs(r.borrow().left.clone(), false, steps + 1, longest_path);
                dfs(r.borrow().right.clone(), true, 1, longest_path);
            } else {
                dfs(r.borrow().right.clone(), true, steps + 1, longest_path);
                dfs(r.borrow().left.clone(), false, 1, longest_path);
            }
        }
    }
    let mut longest_path = 0;
    dfs(root.clone(), true, 0, &mut longest_path);
    dfs(root, false, 0, &mut longest_path);
    longest_path
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut min: i32, mut max: i32) -> i32 {
        if root.is_none() {
            return max - min;
        }
        let root = root.clone().unwrap();
        let root = root.borrow();

        min = min.min(root.val);
        max = max.max(root.val);

        let left = dfs(root.left.clone(), min, max);
        let right = dfs(root.right.clone(), min, max);

        return left.max(right);
    }

    if root.is_none() {
        return 0;
    }

    let x = root.clone().unwrap();
    let x = x.borrow();
    dfs(root, x.val, x.val)
}

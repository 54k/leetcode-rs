#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, leafs: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let root = root.clone().unwrap();
        let root = root.borrow();

        if root.left.is_none() && root.right.is_none() {
            leafs.push(root.val);
        }

        dfs(root.left.clone(), leafs);
        dfs(root.right.clone(), leafs);
    }

    let mut leafs1 = vec![];
    let mut leafs2 = vec![];

    dfs(root1, &mut leafs1);
    dfs(root2, &mut leafs2);
    leafs1 == leafs2
}

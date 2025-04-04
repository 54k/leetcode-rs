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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    type Node = Option<Rc<RefCell<TreeNode>>>;

    fn dfs(root: Node) -> (Node, i32) {
        if let Some(node) = root {
            let left = dfs(node.borrow().left.clone());
            let right = dfs(node.borrow().right.clone());

            if left.1 > right.1 {
                return (left.0, left.1 + 1);
            }
            if left.1 < right.1 {
                return (right.0, right.1 + 1);
            }
            return (Some(node), left.1 + 1);
        }
        return (None, 0);
    }

    dfs(root.clone()).0
}
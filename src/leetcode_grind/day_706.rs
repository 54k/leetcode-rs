// https://leetcode.com/problems/cousins-in-binary-tree-ii/description/?envType=daily-question&envId=2024-10-23
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

pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    type N = Option<Rc<RefCell<TreeNode>>>;
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    let mut prev_lvl_sum = root.clone().unwrap().borrow().val;
    q.push_back(root.clone());
    while (q.len() > 0) {
        let mut current_lvl_sum = 0;
        let lvl_n = q.len();
        for _ in 0..lvl_n {
            let curr_node = q.pop_front().unwrap().unwrap();
            let mut curr_node_mut = curr_node.borrow_mut();
            curr_node_mut.val = prev_lvl_sum - curr_node_mut.val;
            let mut sibling_sum = 0;
            if let Some(right) = curr_node_mut.right.clone() {
                sibling_sum += right.borrow().val;
            }
            if let Some(left) = curr_node_mut.left.clone() {
                sibling_sum += left.borrow().val;
            }

            if !curr_node_mut.left.is_none() {
                current_lvl_sum += curr_node_mut.left.clone().unwrap().borrow().val;
                q.push_back(curr_node_mut.left.clone());
                curr_node_mut.left.clone().unwrap().borrow_mut().val = sibling_sum;
            }
            if !curr_node_mut.right.is_none() {
                current_lvl_sum += curr_node_mut.right.clone().unwrap().borrow().val;
                q.push_back(curr_node_mut.right.clone());
                curr_node_mut.right.clone().unwrap().borrow_mut().val = sibling_sum;
            }
        }
        prev_lvl_sum = current_lvl_sum;
    }
    root
}

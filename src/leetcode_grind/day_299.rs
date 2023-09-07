// https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/
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

pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::collections::HashMap;
    let mut lvl = vec![root];
    let mut swaps = 0;

    while lvl.len() > 0 {
        let mut sorted = vec![];
        let mut idx = HashMap::new();

        for (i, node) in lvl.clone().into_iter().enumerate() {
            if let Some(node) = node {
                sorted.push(node.borrow().val);
                idx.insert(node.borrow().val, i);
            }
        }
        sorted.sort();

        let mut visited = vec![false; sorted.len()];
        for i in 0..sorted.len() {
            if visited[i] || idx[&sorted[i]] == i {
                continue;
            }

            let mut cycle_len = 0;
            let mut j = i;
            while !visited[j] {
                visited[j] = true;
                cycle_len += 1;
                j = idx[&sorted[j]];
            }
            if cycle_len > 0 {
                swaps += cycle_len - 1;
            }
        }

        let mut next = vec![];
        for node in lvl {
            if let Some(node) = node {
                let node = node.borrow();

                if node.left.is_some() {
                    next.push(node.left.clone());
                }

                if node.right.is_some() {
                    next.push(node.right.clone());
                }
            }
        }

        lvl = next;
    }

    swaps
}

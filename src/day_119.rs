// todo https://leetcode.com/problems/strong-password-checker/description/
// todo https://leetcode.com/problems/replace-non-coprime-numbers-in-array/description/

use std::cell::RefCell;
use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Definition for a binary tree node.
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

// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/
pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn inorder(list_head: &mut Option<Box<ListNode>>, len: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if list_head.is_none() {
            return None;
        }

        let left = inorder(list_head, len / 2);
        if let Some(lh) = list_head {
            let mut root = TreeNode::new(lh.val);
            root.left = left;
            *list_head = lh.next.take();
            root.right = inorder(list_head, len - (len / 2) - 1);
            Some(Rc::new(RefCell::new(root)))
        } else {
            None
        }
    }

    let mut len = 0;
    let mut cur = &head;
    while let Some(c) = cur {
        cur = &c.next;
        len += 1;
    }
    inorder(&mut head, len)
}

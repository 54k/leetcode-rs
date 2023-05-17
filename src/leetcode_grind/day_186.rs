use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/delete-node-in-a-bst/description/
pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    type Node = Option<Rc<RefCell<TreeNode>>>;

    fn delete_node(root: Node, key: i32) -> Node {
        fn succ(root: Node) -> i32 {
            let mut root = root.unwrap().borrow().right.clone();
            while root.clone().unwrap().borrow().left.is_some() {
                root = root.unwrap().borrow().left.clone();
            }
            root.unwrap().borrow().val
        }
        fn predecessor(root: Node) -> i32 {
            let mut root = root.unwrap().borrow().left.clone();
            while root.clone().unwrap().borrow().right.is_some() {
                root = root.unwrap().borrow().right.clone();
            }
            root.unwrap().borrow().val
        }

        if let Some(r) = root.clone() {
            if r.borrow().val < key {
                let d = delete_node(r.borrow().right.clone(), key);
                r.borrow_mut().right = d;
                return root;
            } else if r.borrow().val > key {
                let d = delete_node(r.borrow().left.clone(), key);
                r.borrow_mut().left = d;
                return root;
            } else {
                if r.borrow().left.is_none() && r.borrow().right.is_none() {
                    return None;
                } else if r.borrow().right.is_some() {
                    r.borrow_mut().val = succ(root.clone());
                    let d = delete_node(r.borrow().right.clone(), r.borrow().val);
                    r.borrow_mut().right = d;
                } else {
                    r.borrow_mut().val = predecessor(root.clone());
                    let d = delete_node(r.borrow().left.clone(), r.borrow().val);
                    r.borrow_mut().left = d;
                }
            }
        }
        root
    }

    delete_node(root, key)
}

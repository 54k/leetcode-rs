use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/check-completeness-of-a-binary-tree/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn bfs_approach(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        use std::collections::VecDeque;
        let mut null_found = false;
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if node.is_none() {
                null_found = true;
            } else {
                if null_found {
                    return false;
                }
                let node = node.unwrap();
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow().right.clone());
            }
        }
        true
    }
    fn dfs_approach(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
            if let Some(r) = root {
                let r = r.borrow();
                count_nodes(r.left.clone()) + count_nodes(r.right.clone()) + 1
            } else {
                0
            }
        }
        fn check_indicies(root: Option<Rc<RefCell<TreeNode>>>, idx: usize, n: usize) -> bool {
            if let Some(r) = root {
                if idx >= n {
                    return false;
                }
                let r = r.borrow();
                check_indicies(r.left.clone(), idx * 2 + 1, n)
                    && check_indicies(r.right.clone(), idx * 2 + 2, n)
            } else {
                true
            }
        }
        check_indicies(root.clone(), 0, count_nodes(root))
    }
    bfs_approach(root)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test349() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{}", is_complete_tree(root));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        println!("{}", is_complete_tree(root));
    }
}

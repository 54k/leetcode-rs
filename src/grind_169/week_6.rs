use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn dfs_approach(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, height: usize, levels: &mut Vec<Vec<i32>>) {
            if let Some(r) = root {
                if levels.len() == height {
                    levels.push(vec![]);
                }
                dfs(r.as_ref().borrow().left.clone(), height + 1, levels);
                dfs(r.as_ref().borrow().right.clone(), height + 1, levels);
                levels[height].push(r.as_ref().borrow().val);
            }
        }
        dfs(root, 0, &mut ans);
        for i in 0..ans.len() {
            if i % 2 == 1 {
                ans[i].reverse();
            }
        }
        ans
    }
    fn bfs_approach(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut ans = vec![];
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        while !queue.is_empty() {
            ans.push(vec![]);
            let mut n = queue.len();
            while n > 0 {
                if let Some(Some(r)) = queue.pop_front() {
                    ans.last_mut().unwrap().push(r.as_ref().borrow().val);
                    let left = r.as_ref().borrow().left.clone();
                    if left.is_some() {
                        queue.push_back(left);
                    }
                    let right = r.as_ref().borrow().right.clone();
                    if right.is_some() {
                        queue.push_back(right);
                    }
                }
                n -= 1;
            }
        }
        for i in 0..ans.len() {
            if i % 2 == 1 {
                ans[i].reverse();
            }
        }
        ans
    }
    bfs_approach(root)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zigzag_level_order() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{:?}", zigzag_level_order(root));
    }
}

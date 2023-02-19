use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    use std::collections::*;

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, levels: &mut Vec<VecDeque<i32>>, depth: usize) {
        if let Some(r) = root {
            if levels.len() == depth {
                levels.push(VecDeque::new())
            }
            let r = r.borrow();
            if depth % 2 == 0 {
                levels[depth].push_back(r.val);
            } else {
                levels[depth].push_front(r.val);
            }
            dfs(r.left.clone(), levels, depth + 1);
            dfs(r.right.clone(), levels, depth + 1);
        }
    }

    let mut ans = vec![];
    dfs(root, &mut ans, 0);
    ans.into_iter()
        .map(|x| x.into_iter().collect::<Vec<_>>())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test251() {
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

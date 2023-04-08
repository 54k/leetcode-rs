use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/binary-tree-vertical-order-traversal/description/
// https://leetcode.com/problems/binary-tree-vertical-order-traversal/editorial/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    use std::collections::{HashMap, VecDeque};
    let mut map = HashMap::new();
    let mut queue = VecDeque::new();
    if let Some(r) = root {
        queue.push_back((r, 0));
    }
    let (mut min, mut max) = (i32::MAX, i32::MIN);
    while let Some((node, lvl)) = queue.pop_front() {
        min = min.min(lvl);
        max = max.max(lvl);
        map.entry(lvl).or_insert(vec![]).push(node.borrow().val);
        if let Some(left) = node.borrow().left.clone() {
            queue.push_back((left, lvl - 1));
        }
        if let Some(right) = node.borrow().right.clone() {
            queue.push_back((right, lvl + 1));
        }
    }
    let mut ans = vec![];
    for i in min..=max {
        ans.push(map.remove(&i).unwrap());
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test417() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        println!("{:?}", vertical_order(root));
    }
}

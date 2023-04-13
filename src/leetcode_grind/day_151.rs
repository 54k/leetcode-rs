// https://leetcode.com/problems/find-largest-value-in-each-tree-row/description/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    if root.is_some() {
        queue.push_back(root);
    }
    let mut ans = vec![];
    let mut lvl = 0;
    while !queue.is_empty() {
        let mut n = queue.len();
        if lvl == ans.len() {
            ans.push(i32::MIN);
        }
        while n > 0 {
            n -= 1;
            if let Some(node) = queue.pop_front() {
                ans[lvl] = ans[lvl].max(node.as_ref().unwrap().borrow().val);
                let left = node.as_ref().unwrap().borrow().left.clone();
                if left.is_some() {
                    queue.push_back(left);
                }
                let right = node.as_ref().unwrap().borrow().right.clone();
                if right.is_some() {
                    queue.push_back(right);
                }
            }
        }
        lvl += 1;
    }
    ans
}

// https://leetcode.com/problems/deepest-leaves-sum/editorial/
pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    if root.is_some() {
        queue.push_back(root);
    }
    let mut ans = 0;
    while !queue.is_empty() {
        let mut sum = 0;
        for _ in 0..queue.len() {
            if let Some(node) = queue.pop_front() {
                let node = node.as_ref().unwrap().borrow();
                sum += node.val;
                if node.left.is_some() {
                    queue.push_back(node.left.clone());
                }
                if node.right.is_some() {
                    queue.push_back(node.right.clone());
                }
            }
        }
        ans = sum;
    }
    ans
}

// https://leetcode.com/problems/insert-into-a-binary-search-tree/description/
pub fn insert_into_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn insert(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root.clone() {
            if val < r.as_ref().borrow().val {
                let left = r.as_ref().borrow().left.clone();
                r.borrow_mut().left = insert(left, val);
            } else {
                let right = r.as_ref().borrow().right.clone();
                r.borrow_mut().right = insert(right, val);
            }
            root
        } else {
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right: None,
            })))
        }
    }
    insert(root, val)
}

// https://leetcode.com/problems/closest-binary-search-tree-value/description/
// https://leetcode.com/problems/closest-binary-search-tree-value/editorial/
pub fn closest_value(mut root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
    let mut node = root.clone();
    let mut closest = root.as_ref().unwrap().borrow().val;
    while let Some(n) = node {
        if (n.borrow().val as f64 - target).abs() <= (closest as f64 - target).abs() {
            closest = n.borrow().val;
        }
        if target < n.borrow().val as f64 {
            node = n.borrow().left.clone();
        } else {
            node = n.borrow().right.clone();
        }
    }
    closest
}

// https://leetcode.com/problems/basic-calculator-iii/description/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test432() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{:?}", largest_values(root)); // 1,3,9
    }

    #[test]
    fn test433() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{:?}", deepest_leaves_sum(root)); // 17
    }
}

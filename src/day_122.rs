// todo https://leetcode.com/problems/strong-password-checker/description/
// todo https://leetcode.com/problems/replace-non-coprime-numbers-in-array/description/

// https://leetcode.com/problems/sum-root-to-leaf-numbers/
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn preorder(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, total_sum: &mut i32) {
        if let Some(r) = root {
            let r = r.borrow();
            if r.left.is_none() && r.right.is_none() {
                *total_sum += 10 * sum + r.val;
            }
            preorder(r.left.clone(), 10 * sum + r.val, total_sum);
            preorder(r.right.clone(), 10 * sum + r.val, total_sum);
        }
    }
    let mut total_sum = 0;
    preorder(root, 0, &mut total_sum);
    total_sum
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test346() {
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
        println!("{}", sum_numbers(root));
    }
}

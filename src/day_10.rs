use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn num_squares(n: i32) -> i32 {
    let mut dp = vec![0; (n + 1) as usize];
    for i in 1..=n {
        let mut min = i32::MAX;
        let mut j = 1;
        while i - j * j >= 0 {
            min = std::cmp::min(min, dp[(i - j * j) as usize] + 1);
            j += 1
        }
        dp[i as usize] = min;
    }
    dp[n as usize]
}

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

type R = Option<Rc<RefCell<TreeNode>>>;

#[allow(dead_code)]
pub fn flatten(root: &mut R) {
    fn go(root: &mut R, pre: &mut R) -> R {
        if root.is_none() {
            return pre.clone();
        }

        let mut pre = go(&mut root.clone().unwrap().borrow_mut().right, pre);
        let mut pre = go(&mut root.clone().unwrap().borrow_mut().left, &mut pre);

        root.clone().unwrap().borrow_mut().right = pre.clone();
        root.clone().unwrap().borrow_mut().left.take();

        pre = root.clone();
        pre
    }

    go(root, &mut None);
}

#[allow(dead_code)]
pub fn print_postorder_reverse(root: &mut R) {
    if root.is_none() {
        return;
    }

    print_postorder_reverse(&mut root.clone().unwrap().borrow_mut().right);
    print_postorder_reverse(&mut root.clone().unwrap().borrow_mut().left);

    println!("{}", root.clone().unwrap().borrow_mut().val);
}

#[cfg(test)]
mod test {
    use crate::day_10::*;

    #[test]
    fn test59() {
        println!("{}", num_squares(12));
    }

    #[test]
    fn test60() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                val: 2,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                val: 5,
            }))),
        })));
        flatten(&mut root);
        println!("{:?}", root);
        print_postorder_reverse(&mut root);
    }
}

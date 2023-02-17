use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/minimum-distance-between-bst-nodes/description/
pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn min_dist(
        root: Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
        diff: &mut i32,
    ) {
        if let Some(r) = root.clone() {
            let r = r.borrow();
            let val = r.val;

            min_dist(r.left.clone(), prev, diff);
            if let Some(p) = prev {
                *diff = (*diff).min(val - p.borrow().val);
            }
            *prev = root;
            min_dist(r.right.clone(), prev, diff);
        }
    }
    let mut ans = i32::MAX;
    min_dist(root, &mut None, &mut ans);
    ans
}

// https://leetcode.com/problems/binary-tree-tilt/
pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn tilt(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            let v = r.val;
            let lv = tilt(r.left.clone(), ans);
            let rv = tilt(r.right.clone(), ans);
            *ans += (lv - rv).abs();
            v + lv + rv
        } else {
            0
        }
    }
    let mut ans = 0;
    tilt(root, &mut ans);
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test231() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 27,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 34,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 58,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 50,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 44,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
        })));
        println!("{}", min_diff_in_bst(root)); // 6

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 236,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 104,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 227,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 701,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 911,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{}", min_diff_in_bst(root)); // 9
    }

    #[test]
    fn test232() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        println!("{}", find_tilt(root)); // 1
    }
}

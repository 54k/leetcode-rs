use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn safe(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut s = &head;
        let mut f = &head;
        while f.is_some() && f.as_ref().unwrap().next.is_some() {
            f = &f.as_ref().unwrap().next.as_ref().unwrap().next;
            s = &s.as_ref().unwrap().next;
        }
        s.clone()
    }

    fn un_safe(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        unsafe {
            let mut s = Box::into_raw(head?);
            let mut f = s.as_ref();
            while f.is_some() && f.unwrap().next.is_some() {
                f = f?.next.as_ref()?.next.as_deref();
                s = Box::into_raw((*s).next.take()?);
            }
            Some(Box::from_raw(s))
        }
    }

    un_safe(head)
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

#[allow(dead_code)]
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() || root == p || root == q {
        return root;
    }

    let left = lowest_common_ancestor(
        root.clone().unwrap().borrow().left.clone(),
        p.clone(),
        q.clone(),
    );
    let right = lowest_common_ancestor(root.clone().unwrap().borrow().right.clone(), p, q);

    if left.is_some() && right.is_some() {
        return root;
    }

    if left.is_some() {
        left
    } else {
        right
    }
}

#[allow(dead_code)]
pub fn lowest_common_ancestor2(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() || root == p || root == q {
        return root;
    }
    let left = lowest_common_ancestor2(root.clone()?.borrow().left.clone(), p.clone(), q.clone());
    let right = lowest_common_ancestor2(root.clone()?.borrow().right.clone(), p, q);
    if left.is_some() && right.is_some() {
        return root;
    }

    if left.is_some() {
        left
    } else {
        right
    }
}

#[allow(dead_code)]
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(nums: &Vec<i32>, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }
        let mid = (left + right) / 2;
        let mut root = TreeNode {
            val: nums[mid as usize],
            left: None,
            right: None,
        };
        root.left = helper(&nums, left, mid - 1);
        root.right = helper(&nums, mid + 1, right);
        Some(Rc::new(RefCell::new(root)))
    }

    helper(&nums, 0, (nums.len() - 1) as i32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test96() {
        println!(
            "{:?}",
            middle_node(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                }))
            })))
        );
    }

    #[test]
    fn test97() {
        let p = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: p.clone(),
                val: 2,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: q.clone(),
                val: 2,
            }))),
        })));

        println!("{:?}", lowest_common_ancestor(root, p.clone(), q.clone()));
    }

    #[test]
    fn test98() {
        let p = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: p.clone(),
                val: 2,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: q.clone(),
                val: 2,
            }))),
        })));

        println!("{:?}", lowest_common_ancestor2(root, p.clone(), q.clone()));
    }

    #[test]
    fn test99() {
        println!("{:?}", sorted_array_to_bst(vec![-10, -3, 0, 5, 9]));
    }
}

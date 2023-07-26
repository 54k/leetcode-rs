use std::{cell::RefCell, rc::Rc};

// https://leetcode.com/problems/the-latest-time-to-catch-a-bus/description/
pub fn latest_time_catch_the_bus(
    mut buses: Vec<i32>,
    mut passengers: Vec<i32>,
    capacity: i32,
) -> i32 {
    use std::collections::HashSet;
    buses.sort();
    passengers.sort();
    let mut set = HashSet::new();
    let mut ans = 0;
    let mut j = 0;
    for i in 0..buses.len() {
        let mut c = 0;

        while j < passengers.len() && c < capacity && passengers[j] <= buses[i] {
            if !set.contains(&(passengers[j] - 1)) {
                ans = passengers[j] - 1;
            }

            set.insert(passengers[j]);
            j += 1;
            c += 1;
        }
        if c < capacity && !set.contains(&buses[i]) {
            ans = buses[i];
        }
    }
    ans
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack = vec![];
    let mut result = vec![];
    let mut curr = root.clone();

    while stack.len() > 0 || curr.is_some() {
        while let Some(c) = curr {
            stack.push(Some(c.clone()));
            curr = c.borrow().left.clone();
        }
        curr = stack.pop().unwrap().clone();
        result.push(curr.clone().as_ref().unwrap().borrow().val);

        curr = curr.clone().as_ref().unwrap().borrow().right.clone();
    }

    result
}

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut ans = VecDeque::new();
    let mut stack = vec![];
    if root.is_none() {
        return vec![];
    }
    stack.push(root);
    while let Some(top) = stack.pop() {
        ans.push_front(top.clone().as_ref().unwrap().borrow().val);
        if top.clone().as_ref().unwrap().borrow().left.is_some() {
            stack.push(top.clone().as_ref().unwrap().borrow().left.clone());
        }
        if top.clone().as_ref().unwrap().borrow().right.is_some() {
            stack.push(top.clone().as_ref().unwrap().borrow().right.clone());
        }
    }
    ans.into_iter().collect()
}

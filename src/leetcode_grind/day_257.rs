use std::{cell::RefCell, rc::Rc};

// https://leetcode.com/problems/maximum-running-time-of-n-computers/description/
pub fn max_run_time(n: i32, mut batteries: Vec<i32>) -> i64 {
    batteries.sort();
    let bn = batteries.len();

    let mut live = batteries
        .iter()
        .copied()
        .skip(bn - n as usize)
        .map(|x| x as i64)
        .collect::<Vec<_>>();

    let mut extra = batteries
        .iter()
        .copied()
        .take(bn - n as usize)
        .map(|x| x as i64)
        .sum::<i64>();

    for i in 0..n as usize - 1 {
        if extra < ((i + 1) as i64) * (live[i + 1] - live[i]) {
            return live[i] + extra / (i as i64 + 1);
        }

        extra -= (i as i64 + 1) * (live[i + 1] - live[i]);
    }

    live[n as usize - 1] + extra / n as i64
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/symmetric-tree/description/
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    q.push_back(root.clone());
    q.push_back(root.clone());

    while q.len() > 0 {
        let t1 = q.pop_front().unwrap();
        let t2 = q.pop_front().unwrap();
        if t1.is_none() && t2.is_none() {
            continue;
        }
        if t1.is_none() || t2.is_none() {
            return false;
        }
        let t1 = t1.unwrap().clone();
        let t1 = t1.borrow();
        let t2 = t2.unwrap().clone();
        let t2 = t2.borrow();
        if t1.val != t2.val {
            return false;
        }
        q.push_back(t1.left.clone());
        q.push_back(t2.right.clone());
        q.push_back(t1.right.clone());
        q.push_back(t2.left.clone());
    }

    true
}

// https://leetcode.com/problems/path-sum/description/
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if root.is_none() {
        return false;
    }

    let mut node_stack = vec![];
    let mut sum_stack = vec![];
    node_stack.push(root.clone());
    sum_stack.push(target_sum - root.clone().as_ref().unwrap().borrow().val);

    while node_stack.len() > 0 {
        let node = node_stack.pop().unwrap().clone().unwrap();
        let node = node.borrow();
        let curr_sum = sum_stack.pop().unwrap();

        if node.right.is_none() && node.left.is_none() && curr_sum == 0 {
            return true;
        }

        if node.right.is_some() {
            node_stack.push(node.right.clone());
            sum_stack.push(curr_sum - node.right.as_ref().unwrap().borrow().val);
        }
        if node.left.is_some() {
            node_stack.push(node.left.clone());
            sum_stack.push(curr_sum - node.left.as_ref().unwrap().borrow().val);
        }
    }

    false
}

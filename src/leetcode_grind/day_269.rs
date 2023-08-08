use std::{cell::RefCell, rc::Rc};

// https://leetcode.com/problems/trapping-rain-water/description/
pub fn trap_stack(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut stack = vec![];
    let mut current = 0;

    while current < height.len() {
        while !stack.is_empty() && height[stack[stack.len() - 1]] <= height[current] {
            let mid = stack.pop().unwrap();
            if stack.is_empty() {
                break;
            }
            let distance = current - stack[stack.len() - 1] - 1;
            let bounded_height = height[current].min(height[stack[stack.len() - 1]]) - height[mid];
            ans += distance as i32 * bounded_height;
        }

        stack.push(current);
        current += 1;
    }

    ans
}

pub fn trap_2pointers(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut ans = 0;
    let mut left_max = 0;
    let mut right_max = 0;
    while left < right {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                ans += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                ans += right_max - height[right];
            }
            right -= 1;
        }
    }
    ans
}

// https://leetcode.com/problems/validate-binary-search-tree/description/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut prev = i64::MIN;
    let mut stack = vec![];
    let mut curr = root.clone();

    while stack.len() > 0 || curr.is_some() {
        while curr.clone().is_some() {
            stack.push(curr.clone());
            let left = curr.as_ref().unwrap().borrow().left.clone();
            curr = left;
        }

        let top = stack.pop().unwrap();
        let val = top.as_ref().unwrap().borrow().val as i64;
        if val <= prev {
            return false;
        }
        prev = val;
        curr = top.as_ref().unwrap().borrow().right.clone();
    }

    true
}

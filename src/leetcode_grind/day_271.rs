use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/count-univalue-subtrees/description
pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> bool {
        if root.is_none() {
            return true;
        }
        let r = root.as_ref().unwrap();
        let r = r.borrow();
        let left_uni = dfs(r.left.clone(), ans);
        let right_uni = dfs(r.right.clone(), ans);

        if left_uni && right_uni {
            if let Some(left_child) = r.left.clone() {
                if left_child.borrow().val != r.val {
                    return false;
                }
            }
            if let Some(right_child) = r.right.clone() {
                if right_child.borrow().val != r.val {
                    return false;
                }
            }
            *ans += 1;
            return true;
        }
        false
    }
    let mut ans = 0;
    dfs(root, &mut ans);
    ans
}

// https://leetcode.com/problems/maximize-distance-to-closest-person/description/
pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    pub fn max_dist_to_closest_arrays(seats: Vec<i32>) -> i32 {
        let n = seats.len();
        let mut left = vec![n; n];
        let mut right = vec![n; n];

        for i in 0..n {
            if seats[i] == 1 {
                left[i] = 0;
            } else if i > 0 {
                left[i] = left[i - 1] + 1;
            }
        }

        for i in (0..n).rev() {
            if seats[i] == 1 {
                right[i] = 0;
            } else if (i < n - 1) {
                right[i] = right[i + 1] + 1;
            }
        }

        let mut ans = 0;
        for i in 0..n {
            if seats[i] == 0 {
                ans = ans.max(left[i].min(right[i]));
            }
        }
        ans as i32
    }

    pub fn max_dist_to_closest_two_pointers(seats: Vec<i32>) -> i32 {
        let n = seats.len();
        let mut prev = -1;
        let mut future = 0;
        let mut ans = 0;

        for i in 0..n {
            if seats[i] == 1 {
                prev = i as i32;
            } else {
                while future < n && seats[future] == 0 || future < i {
                    future += 1;
                }
                let left = if prev == -1 { n } else { i - prev as usize };
                let right = if future == n { n } else { future - i };

                ans = ans.max(left.min(right));
            }
        }
        ans as i32
    }

    pub fn max_dist_to_closest_groups(seats: Vec<i32>) -> i32 {
        let n = seats.len();
        let mut k = 0;
        let mut ans = 0;

        for i in 0..n {
            if seats[i] == 1 {
                k = 0;
            } else {
                k += 1;
                ans = ans.max((k + 1) / 2)
            }
        }

        for i in 0..n {
            if seats[i] == 1 {
                ans = ans.max(i as i32);
                break;
            }
        }
        for i in (0..n).rev() {
            if seats[i] == 1 {
                ans = ans.max(n as i32 - 1 - i as i32);
                break;
            }
        }
        ans
    }

    max_dist_to_closest_groups(seats)
}

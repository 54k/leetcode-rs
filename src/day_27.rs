#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut min: i32, mut max: i32) -> i32 {
        if root.is_none() {
            return max - min;
        }
        let root = root.clone().unwrap();
        let root = root.borrow();

        min = min.min(root.val);
        max = max.max(root.val);

        let left = dfs(root.left.clone(), min, max);
        let right = dfs(root.right.clone(), min, max);

        return left.max(right);
    }

    if root.is_none() {
        return 0;
    }

    let x = root.clone().unwrap();
    let x = x.borrow();
    dfs(root, x.val, x.val)
}

#[allow(dead_code)]
pub fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
    use std::collections::HashSet;
    let mut prefix = vec![0; nums.len() + 1];
    for i in 1..=nums.len() {
        if nums[i - 1] % p == 0 {
            prefix[i] = prefix[i - 1] + 1;
        } else {
            prefix[i] = prefix[i - 1];
        }
    }

    let mut set = HashSet::new();
    for i in 0..nums.len() {
        for j in i + 1..=nums.len() {
            if prefix[j] - prefix[i] <= k {
                set.insert(&nums[i..j]);
            }
        }
    }

    return set.len() as i32;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test104() {
        println!("{}", count_distinct(vec![2, 3, 3, 2, 2], 2, 2));
        println!("{}", count_distinct(vec![1, 2, 3, 4], 4, 1));
        println!("{}", count_distinct(vec![6, 20, 5, 18], 3, 14));
    }
}

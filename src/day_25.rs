#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    let mut res = 0;
    let mut stack = vec![];
    stack.push(root);

    while !stack.is_empty() {
        let root = stack.pop().unwrap();
        if let Some(r) = root {
            let x = r.borrow();
            if low <= x.val && x.val <= high {
                res += x.val;
            }
            if low < x.val {
                stack.push(x.left.clone());
            }
            if x.val < high {
                stack.push(x.right.clone());
            }
        }
    }
    res
}

#[allow(dead_code)]
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // lolololololol derp derp
    fn lol(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        p == q
    }
    fn dfs(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return if p.is_none() && q.is_none() {
            true
        } else if p.is_some() && q.is_some() {
            let p = p.unwrap();
            let q = q.unwrap();

            let p_borrowed = p.borrow();
            let q_borrowed = q.borrow();

            p_borrowed.val == q_borrowed.val
                && dfs(p_borrowed.left.clone(), q_borrowed.left.clone())
                && dfs(p_borrowed.right.clone(), q_borrowed.right.clone())
        } else {
            false
        };
    }
    dfs(p.clone(), q.clone()) && lol(p, q)
}

#[allow(dead_code)]
pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.clone().unwrap();
        let root = root.borrow();

        let left = dfs(root.left.clone(), max_sum).max(0);
        let right = dfs(root.right.clone(), max_sum).max(0);
        let total = root.val + left + right;

        *max_sum = (*max_sum).max(total);

        (root.val + left).max(root.val + right)
    }
    let mut max = i32::MIN;
    dfs(root, &mut max);
    max
}

#[allow(dead_code)]
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let root = root.clone().unwrap();
        let root = root.borrow();

        let t_sum = target_sum - root.val;

        if root.left.is_none() && root.right.is_none() {
            return t_sum == 0;
        }

        dfs(root.left.clone(), t_sum) || dfs(root.right.clone(), t_sum)
    }

    dfs(root, target_sum)
}

#[allow(dead_code)]
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    fn bfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut ans = vec![];
        let mut q = VecDeque::new();
        q.push_back((root, vec![], target_sum));

        while !q.is_empty() {
            let (e, mut p, t_sum) = q.pop_front().unwrap();

            if e.is_none() {
                continue;
            }

            let e = e.unwrap();
            let e = e.borrow();

            p.push(e.val);

            if e.left.is_none() && e.right.is_none() && t_sum - e.val == 0 {
                ans.push(p.clone());
            }

            if e.left.is_some() {
                q.push_back((e.left.clone(), p.clone(), t_sum - e.val));
            }
            if e.right.is_some() {
                q.push_back((e.right.clone(), p.clone(), t_sum - e.val));
            }
        }

        ans
    }

    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        path: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if root.is_none() {
            return;
        }
        let root = root.unwrap();
        let root = root.borrow();
        path.push(root.val);

        if root.left.is_none() && root.right.is_none() && target_sum - root.val == 0 {
            ans.push(path.clone());
        }

        dfs(root.left.clone(), target_sum - root.val, path, ans);
        dfs(root.right.clone(), target_sum - root.val, path, ans);
        path.pop();
    }

    let mut res = vec![];
    dfs(root.clone(), target_sum, &mut vec![], &mut res);
    assert_eq!(bfs(root, target_sum), res);
    res
}

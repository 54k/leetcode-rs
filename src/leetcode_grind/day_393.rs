// https://leetcode.com/problems/transpose-matrix/description
pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut ans = vec![vec![0; m]; n];
    for r in 0..m {
        for c in 0..n {
            ans[c][r] = matrix[r][c];
        }
    }
    ans
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

// https://leetcode.com/problems/delete-nodes-and-return-forest/description/
use std::cell::RefCell;
use std::rc::Rc;
pub fn del_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    to_delete: Vec<i32>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    use std::collections::HashSet;
    type Node = Option<Rc<RefCell<TreeNode>>>;
    let to_delete = to_delete.into_iter().collect::<HashSet<_>>();
    let mut ans = vec![];

    fn post_order(
        root: Node,
        is_root: bool,
        to_delete: &HashSet<i32>,
        ans: &mut Vec<Node>,
    ) -> Node {
        if root.is_none() {
            return None;
        }

        let r = root.clone().unwrap();
        let mut r = r.borrow_mut();
        let is_deleted = to_delete.contains(&r.val);
        if is_root && !is_deleted {
            ans.push(root.clone());
        }

        r.left = post_order(r.left.clone(), is_deleted, to_delete, ans);
        r.right = post_order(r.right.clone(), is_deleted, to_delete, ans);

        if is_deleted {
            None
        } else {
            root.clone()
        }
    }

    post_order(root.clone(), true, &to_delete, &mut ans);
    ans
}

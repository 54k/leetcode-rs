#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, leafs: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let root = root.clone().unwrap();
        let root = root.borrow();

        if root.left.is_none() && root.right.is_none() {
            leafs.push(root.val);
        }

        dfs(root.left.clone(), leafs);
        dfs(root.right.clone(), leafs);
    }

    let mut leafs1 = vec![];
    let mut leafs2 = vec![];

    dfs(root1, &mut leafs1);
    dfs(root2, &mut leafs2);
    leafs1 == leafs2
}

#[allow(dead_code)]
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::HashMap;

    let inorder_map = inorder
        .into_iter()
        .enumerate()
        .map(|x| (x.1, x.0 as i32))
        .collect::<HashMap<i32, i32>>();

    fn arr_to_tree(
        preorder: &[i32],
        i: &mut usize,
        left: i32,
        right: i32,
        m: &HashMap<i32, i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }

        let mut root = TreeNode {
            val: preorder[*i],
            left: None,
            right: None,
        };

        *i += 1;

        root.left = arr_to_tree(preorder, i, left, m.get(&root.val).unwrap() - 1, m);
        root.right = arr_to_tree(preorder, i, m.get(&root.val).unwrap() + 1, right, m);

        Some(Rc::new(RefCell::new(root)))
    }

    arr_to_tree(
        &preorder[..],
        &mut 0,
        0,
        preorder.len() as i32 - 1,
        &inorder_map,
    )
}

#[cfg(test)]
mod test {
    use crate::day_26::build_tree;

    #[test]
    fn test102() {
        println!(
            "{:?}",
            build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
        );
    }
}

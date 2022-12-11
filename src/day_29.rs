use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, paths: &mut Vec<String>, path: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }

        let root = root.unwrap();
        let root = root.borrow();
        path.push(root.val);
        if root.right.is_none() && root.left.is_none() {
            let mut result = String::new();
            for (i, x) in path.clone().into_iter().enumerate() {
                if i == 0 {
                    result.push_str(&format!("{}", x));
                } else {
                    result.push_str(&format!("->{}", x));
                }
            }
            paths.push(result);
        }
        dfs(root.left.clone(), paths, path);
        dfs(root.right.clone(), paths, path);
        path.pop();
    }
    let mut result = vec![];
    dfs(root, &mut result, &mut vec![]);
    result
}

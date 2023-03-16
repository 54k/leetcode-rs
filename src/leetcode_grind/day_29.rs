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

#[allow(dead_code)]
pub fn my_pow(x: f64, n: i32) -> f64 {
    fn pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let u = pow(x, n / 2);
        if n % 2 == 0 {
            u * u
        } else {
            x * u * u
        }
    }
    let p = pow(x, n.abs());
    if n < 0 {
        1.0 / p
    } else {
        p
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test104() {
        println!("{}", my_pow(2.0, 3));
        println!("{}", my_pow(2.0, -2));
        println!("{}", my_pow(34.00515, -3));
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::HashMap;
    let inorder_map = inorder
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i as i32))
        .collect::<HashMap<i32, i32>>();
    fn construct(
        postoder: &[i32],
        idx: &mut usize,
        inorder_map: &HashMap<i32, i32>,
        left: i32,
        right: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }
        let mut root = TreeNode {
            val: postoder[*idx],
            left: None,
            right: None,
        };
        if *idx > 0 {
            *idx -= 1;
        }

        root.right = construct(
            postoder,
            idx,
            inorder_map,
            inorder_map[&root.val] + 1,
            right,
        );
        root.left = construct(postoder, idx, inorder_map, left, inorder_map[&root.val] - 1);

        Some(Rc::new(RefCell::new(root)))
    }
    construct(
        &postorder,
        &mut (postorder.len() - 1),
        &inorder_map,
        0,
        postorder.len() as i32 - 1,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test351() {
        println!(
            "{:?}",
            build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3])
        );
    }
}

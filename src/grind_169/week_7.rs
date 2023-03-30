// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn preorder_serialize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
            if let Some(r) = root {
                format!(
                    "{},{},{}",
                    r.as_ref().borrow().val,
                    preorder_serialize(r.as_ref().borrow().left.clone()),
                    preorder_serialize(r.as_ref().borrow().right.clone()),
                )
            } else {
                "X".to_string()
            }
        }
        preorder_serialize(root)
    }
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn preorder_deserialize(data: &Vec<&str>, i: &mut i32) -> Option<Rc<RefCell<TreeNode>>> {
            *i += 1;
            let val = data[*i as usize];
            if val == "X" {
                return None;
            }
            let root = TreeNode {
                val: val.parse().unwrap(),
                left: preorder_deserialize(data, i),
                right: preorder_deserialize(data, i),
            };
            Some(Rc::new(RefCell::new(root)))
        }
        preorder_deserialize(&data.split(',').collect(), &mut -1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_codec() {
        // it works
    }
}

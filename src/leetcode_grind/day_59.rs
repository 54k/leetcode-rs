// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, d: i32) -> i32 {
        if root.is_none() {
            return d;
        }
        let r = root.clone().unwrap();
        let left = r.borrow().left.clone();
        let right = r.borrow().right.clone();
        d.max(dfs(left, d + 1)).max(dfs(right, d + 1))
    }
    dfs(root, 0)
}

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn h(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let r = root.unwrap();
        let left = r.borrow().left.clone();
        let right = r.borrow().right.clone();
        let hl = h(left);
        let hr = h(right);
        if hl < 0 || hr < 0 || (hl - hr).abs() > 1 {
            return -1;
        }
        hl.max(hr) + 1
    }

    if root.is_none() {
        return true;
    }
    h(root) > -1
}

// https://leetcode.com/problems/subtree-of-another-tree/solutions/2645723/subtree-of-another-tree/
pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }
        if is_same(root.clone(), sub_root.clone()) {
            return true;
        }
        dfs(
            root.clone().unwrap().borrow().left.clone(),
            sub_root.clone(),
        ) || dfs(root.unwrap().borrow().right.clone(), sub_root)
    }
    fn is_same(l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if l.is_none() && r.is_none() {
            return true;
        }
        if let (Some(l), Some(r)) = (l, r) {
            return l.borrow().val == r.borrow().val
                && is_same(l.borrow().left.clone(), r.borrow().left.clone())
                && is_same(l.borrow().right.clone(), r.borrow().right.clone());
        }
        false
    }
    dfs(root, sub_root)
}

struct Codec {}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn s(root: Option<Rc<RefCell<TreeNode>>>) -> String {
            if root.is_none() {
                return "null".to_string();
            }
            let r = root.unwrap();
            let r = r.borrow();
            format!("{},{},{}", r.val, s(r.left.clone()), s(r.right.clone()))
        }
        s(root)
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn h(data: &Vec<&str>, i: &mut i32) -> Option<Rc<RefCell<TreeNode>>> {
            *i += 1;
            if data[*i as usize] == "null" {
                return None;
            }
            let mut n = TreeNode {
                val: data[*i as usize].parse().unwrap(),
                left: None,
                right: None,
            };
            n.left = h(data, i);
            n.right = h(data, i);
            Some(Rc::new(RefCell::new(n)))
        }
        let data = data.split(',').collect::<Vec<_>>();
        h(&data, &mut -1)
    }
}
// impl Codec {
//     fn new() -> Self {
//         Codec {}
//     }
//
//     fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
//         const INF: i32 = i32::MIN;
//         if root.is_none() {
//             return "".to_string();
//         }
//         fn lvl(root: Option<Rc<RefCell<TreeNode>>>, s: &mut Vec<i32>, d: usize) {
//             while s.len() <= d * 2 + 1 {
//                 s.push(INF);
//             }
//             if let Some(r) = root {
//                 lvl(r.borrow().left.clone(), s, d * 2);
//                 lvl(r.borrow().right.clone(), s, d * 2 + 1);
//                 s[d] = r.borrow().val;
//             } else {
//                 s[d] = INF;
//             }
//         }
//         let mut v = vec![INF];
//         lvl(root, &mut v, 1);
//         while !v.is_empty() && v[v.len() - 1] == INF {
//             v.pop();
//         }
//         v.into_iter()
//             .skip(1)
//             .map(|i| {
//                 if i > INF {
//                     i.to_string()
//                 } else {
//                     "null".to_string()
//                 }
//             })
//             .collect::<Vec<_>>()
//             .join(",")
//     }
//
//     fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
//         if data.is_empty() {
//             return None;
//         }
//         let data = data
//             .split(',')
//             .filter(|s| !s.is_empty())
//             .collect::<Vec<_>>();
//         let mut v = vec![None; data.len() + 1];
//         for (i, n) in data.into_iter().enumerate() {
//             let i = i + 1;
//             v[i] = match n {
//                 "null" => None,
//                 _ => Some(Rc::new(RefCell::new(TreeNode {
//                     val: n.parse().unwrap(),
//                     left: None,
//                     right: None,
//                 }))),
//             };
//             if i > 1 {
//                 if i % 2 == 0 {
//                     if let Some(p) = v[i / 2].clone() {
//                         p.borrow_mut().left = v[i].clone();
//                     }
//                 } else if let Some(p) = v[(i - 1) / 2].clone() {
//                     p.borrow_mut().right = v[i].clone();
//                 }
//             }
//         }
//         v[1].clone()
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test153() {
        let codec = Codec::new();
        let tree: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        let serialized = codec.serialize(tree);
        println!("{}", serialized);
        let deserialized = codec.deserialize(serialized);
        println!("{:?}", deserialized);
    }
}

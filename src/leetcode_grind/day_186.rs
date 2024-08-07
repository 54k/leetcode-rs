use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/delete-node-in-a-bst/description/
pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    type Node = Option<Rc<RefCell<TreeNode>>>;

    fn delete_node(root: Node, key: i32) -> Node {
        fn succ(root: Node) -> i32 {
            let mut root = root.unwrap().borrow().right.clone();
            while root.clone().unwrap().borrow().left.is_some() {
                root = root.unwrap().borrow().left.clone();
            }
            root.unwrap().borrow().val
        }
        fn predecessor(root: Node) -> i32 {
            let mut root = root.unwrap().borrow().left.clone();
            while root.clone().unwrap().borrow().right.is_some() {
                root = root.unwrap().borrow().right.clone();
            }
            root.unwrap().borrow().val
        }

        if let Some(r) = root.clone() {
            if r.borrow().val < key {
                let d = delete_node(r.borrow().right.clone(), key);
                r.borrow_mut().right = d;
                return root;
            } else if r.borrow().val > key {
                let d = delete_node(r.borrow().left.clone(), key);
                r.borrow_mut().left = d;
                return root;
            } else {
                if r.borrow().left.is_none() && r.borrow().right.is_none() {
                    return None;
                } else if r.borrow().right.is_some() {
                    r.borrow_mut().val = succ(root.clone());
                    let d = delete_node(r.borrow().right.clone(), r.borrow().val);
                    r.borrow_mut().right = d;
                } else {
                    r.borrow_mut().val = predecessor(root.clone());
                    let d = delete_node(r.borrow().left.clone(), r.borrow().val);
                    r.borrow_mut().left = d;
                }
            }
        }
        root
    }

    delete_node(root, key)
}

// https://leetcode.com/problems/insert-into-a-binary-search-tree/description/
pub fn insert_into_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn insert_into_bst(root: Node, val: i32) -> Node {
        if let Some(r) = root.clone() {
            if val < r.borrow().val {
                let ins = insert_into_bst(r.borrow().left.clone(), val);
                r.borrow_mut().left = ins;
                root
            } else {
                let ins = insert_into_bst(r.borrow().right.clone(), val);
                r.borrow_mut().right = ins;
                root
            }
        } else {
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right: None,
            })))
        }
    }
    insert_into_bst(root, val)
}

// https://leetcode.com/problems/binary-search-tree-iterator/description/
type Node = Option<Rc<RefCell<TreeNode>>>;

struct BSTIterator {
    stack: Vec<Node>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut ret = Self { stack: vec![] };
        ret.leftmost_intorder(root);
        ret
    }

    fn leftmost_intorder(&mut self, mut root: Node) {
        while root.is_some() {
            self.stack.push(root.clone());
            root = root.unwrap().borrow().left.clone();
        }
    }

    fn next(&mut self) -> i32 {
        let ret = self.stack.pop().unwrap();
        if ret.as_ref().unwrap().borrow().right.is_some() {
            self.leftmost_intorder(ret.as_ref().unwrap().borrow().right.clone());
        }
        ret.unwrap().borrow().val
    }

    fn has_next(&self) -> bool {
        self.stack.len() > 0
    }
}

// https://leetcode.com/problems/search-a-2d-matrix-ii/description/
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    fn search(matrix: &Vec<Vec<i32>>, start: i32, target: i32, search_row: bool) -> bool {
        let mut lo = start;
        let mut hi = if search_row {
            matrix.len() - 1
        } else {
            matrix[0].len() - 1
        } as i32;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if search_row {
                if matrix[mid as usize][start as usize] < target {
                    lo = mid + 1;
                } else if matrix[mid as usize][start as usize] > target {
                    hi = mid - 1;
                } else {
                    return true;
                }
            } else {
                if matrix[start as usize][mid as usize] < target {
                    lo = mid + 1;
                } else if matrix[start as usize][mid as usize] > target {
                    hi = mid - 1;
                } else {
                    return true;
                }
            }
        }

        false
    }

    let shorter_dim = matrix.len().min(matrix[0].len()) as i32;
    for i in 0..shorter_dim {
        let is_vert_found = search(&matrix, i, target, false);
        let is_horizontal_found = search(&matrix, i, target, true);

        if is_vert_found || is_horizontal_found {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test502() {
        println!(
            "{}",
            search_matrix(
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30],
                ],
                5,
            )
        ); // true
    }
}

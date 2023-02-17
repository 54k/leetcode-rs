use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/minimum-distance-between-bst-nodes/description/
pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn min_dist(
        root: Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
        diff: &mut i32,
    ) {
        if let Some(r) = root.clone() {
            let r = r.borrow();
            let val = r.val;

            min_dist(r.left.clone(), prev, diff);
            if let Some(p) = prev {
                *diff = (*diff).min(val - p.borrow().val);
            }
            *prev = root;
            min_dist(r.right.clone(), prev, diff);
        }
    }
    let mut ans = i32::MAX;
    min_dist(root, &mut None, &mut ans);
    ans
}

// https://leetcode.com/problems/binary-tree-tilt/
pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn tilt(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            let v = r.val;
            let lv = tilt(r.left.clone(), ans);
            let rv = tilt(r.right.clone(), ans);
            *ans += (lv - rv).abs();
            v + lv + rv
        } else {
            0
        }
    }
    let mut ans = 0;
    tilt(root, &mut ans);
    ans
}

// https://leetcode.com/problems/minimum-depth-of-binary-tree/
pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            let left_depth = dfs(r.left.clone());
            let right_depth = dfs(r.right.clone());
            if left_depth == 0 || right_depth == 0 {
                left_depth + right_depth + 1
            } else {
                1 + left_depth.min(right_depth)
            }
        } else {
            0
        }
    }
    dfs(root)
}

// https://leetcode.com/problems/root-equals-sum-of-children/description/
pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            let left_sum = dfs(r.left.clone());
            let right_sum = dfs(r.right.clone());
            left_sum + right_sum + r.val
        } else {
            0
        }
    }
    let root_val = root.clone().unwrap().borrow().val;
    let sum_tree = dfs(root);
    sum_tree - root_val == root_val
}

// https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers/
// https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers/solutions/797205/sum-root-to-leaf-binary-numbers/
pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn preorder(root: Option<Rc<RefCell<TreeNode>>>, current_path: i32, root_to_leaf: &mut i32) {
        if let Some(r) = root {
            let r = r.borrow();
            let r_val = r.val;

            let current_path = (current_path << 1) | r_val;
            if r.left.is_none() && r.right.is_none() {
                *root_to_leaf += current_path;
            }
            preorder(r.left.clone(), current_path, root_to_leaf);
            preorder(r.right.clone(), current_path, root_to_leaf);
        }
    }
    let mut ans = 0;
    preorder(root, 0, &mut ans);
    ans
}

// https://leetcode.com/problems/evaluate-boolean-binary-tree/
pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // Leaf nodes have either the value 0 or 1, where 0 represents False and 1 represents True.
    // Non-leaf nodes have either the value 2 or 3, where 2 represents the boolean OR and 3 represents the boolean AND.
    fn postorder(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(r) = root {
            let r = r.borrow();
            let left_val = postorder(r.left.clone());
            let right_val = postorder(r.right.clone());
            if r.left.is_none() && r.right.is_none() {
                // is leaf
                r.val == 1
            } else {
                match r.val {
                    2 => left_val || right_val,
                    3 => left_val && right_val,
                    _ => unreachable!(),
                }
            }
        } else {
            false
        }
    }
    postorder(root)
}

// https://leetcode.com/problems/find-a-corresponding-node-of-a-binary-tree-in-a-clone-of-that-tree/description/
// https://leetcode.com/problems/find-a-corresponding-node-of-a-binary-tree-in-a-clone-of-that-tree/solutions/966198/find-a-corresponding-node-of-a-binary-tree-in-a-clone-of-that-tree/?orderBy=most_relevant
/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} original
 * @param {TreeNode} cloned
 * @param {TreeNode} target
 * @return {TreeNode}
 */
// javascript
// const getTargetCopy = function(original, cloned, target) {
//     let ans = null
//     const inorder = (o, c) => {
//         if (o !== null) {
//             inorder(o.left, c.left)
//             if (o === target) {
//                  ans = c
//             }
//             inorder(o.right, c.right)
//         }
//     }
//     inorder(original, cloned)
//     return ans
// }

// https://leetcode.com/problems/binary-tree-level-order-traversal-ii/
pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::*;
        if root.is_none() {
            return vec![];
        }
        let mut paths = vec![];
        let mut q = VecDeque::new();
        q.push_back(root);
        while !q.is_empty() {
            let mut len = q.len();
            let mut same_level_siblings = vec![];
            while len > 0 {
                len -= 1;
                let node = q.pop_front().unwrap().unwrap();
                let node = node.borrow();
                same_level_siblings.push(node.val);
                if node.left.is_some() {
                    q.push_back(node.left.clone());
                }
                if node.right.is_some() {
                    q.push_back(node.right.clone());
                }
            }
            paths.push(same_level_siblings);
        }
        paths.into_iter().rev().collect()
    }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, paths: &mut Vec<Vec<i32>>) {
            if let Some(r) = root {
                let r = r.borrow();
                if depth == paths.len() {
                    paths.push(vec![]);
                }
                dfs(r.left.clone(), depth + 1, paths);
                dfs(r.right.clone(), depth + 1, paths);
                paths[depth].push(r.val);
            }
        }
        dfs(root, 0, &mut ans);
        ans.into_iter().rev().collect()
    }
    bfs(root)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test231() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 27,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 34,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 58,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 50,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 44,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
        })));
        println!("{}", min_diff_in_bst(root)); // 6

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 236,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 104,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 227,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 701,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 911,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{}", min_diff_in_bst(root)); // 9
    }

    #[test]
    fn test232() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        println!("{}", find_tilt(root)); // 1
    }

    #[test]
    fn test233() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));
        println!("{}", min_depth(root));
    }

    #[test]
    fn test234() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        println!("{}", check_tree(root));
    }

    #[test]
    fn test235() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{}", sum_root_to_leaf(root));
    }

    #[test]
    fn test236() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{}", evaluate_tree(root));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        })));
        println!("{}", evaluate_tree(root));
    }

    #[test]
    fn test237() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        println!("{:?}", level_order_bottom(root)); // [[4], [2, 3], [1]]
    }
}

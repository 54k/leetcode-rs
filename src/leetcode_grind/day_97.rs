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
    dfs(root)
}

// https://leetcode.com/problems/sum-root-to-leaf-numbers/
pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn preorder(
        root: Option<Rc<RefCell<TreeNode>>>,
        mut current_sum: i32,
        sum_leaf_numbers: &mut i32,
    ) {
        if let Some(r) = root {
            let r = r.borrow();
            current_sum *= 10;
            current_sum += r.val;
            if r.left.is_none() && r.right.is_none() {
                *sum_leaf_numbers += current_sum;
            }
            preorder(r.left.clone(), current_sum, sum_leaf_numbers);
            preorder(r.right.clone(), current_sum, sum_leaf_numbers);
        }
    }
    let mut ans = 0;
    preorder(root, 0, &mut ans);
    ans
}

// https://leetcode.com/problems/smallest-string-starting-from-leaf/
pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    fn preorder(root: Option<Rc<RefCell<TreeNode>>>, mut path: String, smallest_path: &mut String) {
        if let Some(r) = root {
            let r = r.borrow();
            path.push(char::from_u32(r.val as u32 + 'a' as u32).unwrap());

            if r.left.is_none() && r.right.is_none() {
                let mut path = path.clone().chars().rev().collect();
                // reached leaf
                if smallest_path.is_empty() {
                    *smallest_path = path;
                } else {
                    *smallest_path = smallest_path.min(&mut path).to_string();
                }
            }

            preorder(r.left.clone(), path.clone(), smallest_path);
            preorder(r.right.clone(), path.clone(), smallest_path);
        }
    }
    let mut ans = "".to_string();
    preorder(root, "".to_string(), &mut ans);
    ans
}

// https://leetcode.com/problems/kth-smallest-element-in-a-bst/
pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    fn kth_smallest_arr(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn inorder(root: Option<Rc<RefCell<TreeNode>>>, k: i32, ans: &mut Vec<i32>) {
            if let Some(r) = root {
                let r = r.borrow();
                inorder(r.left.clone(), k, ans);
                ans.push(r.val);
                inorder(r.right.clone(), k, ans);
            }
        }
        let mut ans = vec![];
        inorder(root, k, &mut ans);
        ans[k as usize - 1]
    }

    fn kth_smallest_elem(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        fn inorder(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32, ans: &mut i32) {
            if let Some(r) = root {
                let r = r.borrow();
                inorder(r.left.clone(), k, ans);
                *k -= 1;
                if *k == 0 {
                    *ans = r.val;
                }
                inorder(r.right.clone(), k, ans);
            }
        }
        let mut ans = 0;
        inorder(root, &mut k, &mut ans);
        ans
    }

    kth_smallest_elem(root, k)
}

// https://leetcode.com/problems/binary-tree-right-side-view/
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn preorder_inverted(
        root: Option<Rc<RefCell<TreeNode>>>,
        depth: i32,
        path: &mut Vec<i32>,
        max_depth: &mut i32,
    ) {
        if let Some(r) = root {
            let r = r.borrow();
            if *max_depth < depth {
                *max_depth = depth;
                path.push(r.val);
            }
            preorder_inverted(r.right.clone(), depth + 1, path, max_depth);
            preorder_inverted(r.left.clone(), depth + 1, path, max_depth);
        }
    }
    let mut max_depth = -1;
    let mut ans = vec![];
    preorder_inverted(root, 0, &mut ans, &mut max_depth);
    ans
}

// https://leetcode.com/problems/sum-of-left-leaves/
pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn postorder(
        root: Option<Rc<RefCell<TreeNode>>>,
        sum_leaves: &mut i32,
        is_left_direction: bool,
    ) {
        if let Some(r) = root {
            let r = r.borrow();
            postorder(r.left.clone(), sum_leaves, true);
            postorder(r.right.clone(), sum_leaves, false);
            if r.left.is_none() && r.right.is_none() && is_left_direction {
                *sum_leaves += r.val;
            }
        }
    }
    fn preorder(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        if let Some(r) = root {
            let r = r.borrow();
            if let Some(l) = r.left.clone() {
                // is it left leaf node?
                let l = l.borrow();
                if l.left.is_none() && l.right.is_none() {
                    sum += l.val;
                }
            }
            sum += preorder(r.left.clone());
            sum += preorder(r.right.clone());
        }
        sum
    }
    // preorder(root);
    let mut ans = 0;
    postorder(root, &mut ans, true);
    ans
}

// https://leetcode.com/problems/convert-bst-to-greater-tree/description/
pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, tree_sum_right: &mut i32) {
        if let Some(r) = root {
            let mut r = r.borrow_mut();
            inorder(r.right.clone(), tree_sum_right);
            *tree_sum_right += r.val;
            r.val += *tree_sum_right;
            inorder(r.left.clone(), tree_sum_right);
        }
    }
    inorder(root.clone(), &mut 0);
    root
}

// https://leetcode.com/problems/trim-a-binary-search-tree/
pub fn trim_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    low: i32,
    high: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn postorder(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn postorder(
            root: Option<Rc<RefCell<TreeNode>>>,
            low: i32,
            high: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(r) = root.clone() {
                let mut r = r.borrow_mut();
                r.left = postorder(r.left.clone(), low, high);
                r.right = postorder(r.right.clone(), low, high);
                if low <= r.val && r.val <= high {
                    root
                } else if r.left.is_some() {
                    r.left.clone()
                } else if r.right.is_some() {
                    r.right.clone()
                } else {
                    None
                }
            } else {
                None
            }
        }
        postorder(root, low, high)
    }

    fn recursive(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root.clone() {
            let mut r = r.borrow_mut();
            if r.val < low {
                return recursive(r.right.clone(), low, high);
            }
            if r.val > high {
                return recursive(r.left.clone(), low, high);
            }

            r.left = recursive(r.left.clone(), low, high);
            r.right = recursive(r.right.clone(), low, high);
            root
        } else {
            None
        }
    }
    recursive(root, low, high)
}

// https://leetcode.com/problems/two-sum-iv-input-is-a-bst/
pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
    use std::collections::*;
    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, k: i32, set: &mut HashSet<i32>) -> bool {
        let mut res = false;
        if let Some(r) = root {
            let r = r.borrow();
            res |= inorder(r.left.clone(), k, set);
            if set.contains(&(k - r.val)) {
                res |= true;
            }
            set.insert(r.val);
            res |= inorder(r.right.clone(), k, set);
            res
        } else {
            res
        }
    }
    let mut set = HashSet::new();
    inorder(root, k, &mut set)
}

// https://leetcode.com/problems/maximum-binary-tree/description/
pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::*;
    let map = nums
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i as i32))
        .collect::<HashMap<i32, i32>>();
    fn dfs(
        nums: &[i32],
        lo: i32,
        hi: i32,
        map: &HashMap<i32, i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if lo <= hi {
            let max = nums[lo as usize..=hi as usize].iter().max().unwrap();
            let max_idx = *map.get(max).unwrap();

            let left = dfs(nums, lo, max_idx - 1, map);
            let right = dfs(nums, max_idx + 1, hi, map);

            Some(Rc::new(RefCell::new(TreeNode {
                val: *max,
                left,
                right,
            })))
        } else {
            None
        }
    }
    dfs(&nums, 0, nums.len() as i32 - 1, &map)
}

// https://leetcode.com/problems/maximum-binary-tree-ii/description/
pub fn insert_into_max_tree(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root.clone() {
            let mut r = r.borrow_mut();
            if r.val > val {
                r.right = dfs(r.right.clone(), val);
                root
            } else {
                Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left: root,
                    right: None,
                })))
            }
        } else {
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right: None,
            })))
        }
    }
    dfs(root, val)
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

    #[test]
    fn test238() {
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
        println!("{}", sum_numbers(root));
    }

    #[test]
    fn test239() {
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
        println!("{}", smallest_from_leaf(root));
    }

    #[test]
    fn test240() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        })));
        println!("{}", kth_smallest(root, 2)); // 2
    }

    #[test]
    fn test241() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        })));
        println!("{:?}", right_side_view(root)); // [3,4,2]
    }

    #[test]
    fn test242() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 10,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        println!("{}", sum_of_left_leaves(root)); // 12

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));

        println!("{}", sum_of_left_leaves(root)); // 1
    }

    #[test]
    fn test243() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));

        println!("{:?}", convert_bst(root));
    }

    #[test]
    fn test244() {
        println!(
            "{:?}",
            construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5])
        );
    }
}

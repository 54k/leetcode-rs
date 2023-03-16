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

use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/diameter-of-binary-tree/description/
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> (i32, i32) {
        if root.is_none() {
            return (0, 0);
        }
        let r = root.unwrap();
        let r = r.borrow();
        let pl = dfs(r.left.clone(), res);
        let pr = dfs(r.right.clone(), res);
        let mut path = pl.0.max(pl.1) + 1 + pr.0.max(pr.1);
        *res = *res.max(&mut path);
        (pl.0.max(pl.1) + 1, pr.0.max(pr.1) + 1)
    }
    let mut res = 0;
    dfs(root, &mut res);
    res - 1
}

// https://leetcode.com/problems/longest-path-with-different-adjacent-characters/description/
pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
    fn dfs(parent: Vec<i32>, s: String) -> i32 {
        fn dfs(v: i32, parent: &Vec<Vec<i32>>, s: &Vec<char>, ans: &mut i32) -> i32 {
            if parent[v as usize].is_empty() {
                return 1;
            }
            let mut longest_chain = 0;
            let mut second_longest_chain = 0;
            for &u in &parent[v as usize] {
                let longest_start_from_child = dfs(u, parent, s, ans);
                if s[v as usize] == s[u as usize] {
                    continue;
                }
                if longest_start_from_child > longest_chain {
                    second_longest_chain = longest_chain;
                    longest_chain = longest_start_from_child;
                } else if longest_start_from_child > second_longest_chain {
                    second_longest_chain = longest_start_from_child;
                }
            }
            *ans = *ans.max(&mut (longest_chain + second_longest_chain + 1));
            longest_chain + 1
        }

        let mut longest_path = 1;
        let mut adj = vec![vec![]; s.len()];
        for (i, p) in parent.into_iter().enumerate().skip(1) {
            adj[p as usize].push(i as i32);
        }
        dfs(0, &adj, &s.chars().collect(), &mut longest_path);
        longest_path
    }

    fn bfs(parent: Vec<i32>, s: String) -> i32 {
        use std::collections::VecDeque;
        let s = s.chars().collect::<Vec<_>>();
        let n = parent.len();

        let mut children_count = vec![0; n];
        for i in 1..n {
            children_count[parent[i] as usize] += 1;
        }

        let mut longest_path = 1;
        let mut longest_chains = vec![vec![0; 2]; n];
        let mut queue = VecDeque::new();

        for i in 1..n {
            if children_count[i] == 0 {
                queue.push_back(i);
                longest_chains[i][0] = 1;
            }
        }

        while let Some(v) = queue.pop_front() {
            let p = parent[v] as usize;

            let longest_chain_from_current_node = longest_chains[v][0];

            if s[v] != s[p] {
                if longest_chain_from_current_node > longest_chains[p][0] {
                    longest_chains[p][1] = longest_chains[p][0];
                    longest_chains[p][0] = longest_chain_from_current_node;
                } else if longest_chain_from_current_node > longest_chains[v][1] {
                    longest_chains[p][1] = longest_chain_from_current_node;
                }
            }

            longest_path = longest_path.max(longest_chains[p][0] + longest_chains[p][1] + 1);
            children_count[p] -= 1;
            if children_count[p] == 0 && p != 0 {
                longest_chains[p][0] += 1;
                queue.push_back(p);
            }
        }
        longest_path
    }
    bfs(parent, s)
}

// https://leetcode.com/problems/longest-univalue-path/
pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    const INF: i32 = i32::MIN;
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> (i32, i32) {
        if root.is_none() {
            return (INF, 0);
        }

        let root = root.unwrap();
        let root = root.borrow();

        let mut arrow_left = 0;
        let mut arrow_right = 0;

        let (val_left, left) = dfs(root.left.clone(), ans);
        let (val_right, right) = dfs(root.right.clone(), ans);

        if root.val == val_left {
            arrow_left += left + 1;
        }
        if root.val == val_right {
            arrow_right += right + 1;
        }
        *ans = *ans.max(&mut (arrow_left + arrow_right));
        (root.val, arrow_left.max(arrow_right))
    }
    let mut ans = 0;
    dfs(root, &mut ans);
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test156() {
        let tree: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        println!("{}", diameter_of_binary_tree(tree));

        let tree2: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: None,
        })));

        println!("{}", diameter_of_binary_tree(tree2));
    }

    #[test]
    fn test157() {
        println!(
            "{}",
            longest_path(vec![-1, 0, 0, 1, 1, 2], "abacbe".to_string())
        ); // 3
        println!("{}", longest_path(vec![-1, 0, 0, 0], "aabc".to_string())); // 3

        println!("{}", longest_path(vec![-1], "z".to_string())); // 1
    }

    #[test]
    fn test158() {
        let tree: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
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
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{}", longest_univalue_path(tree));

        let tree2: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{}", longest_univalue_path(tree2));

        let tree3: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
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
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
        })));
        println!("{}", longest_univalue_path(tree3));
    }
}

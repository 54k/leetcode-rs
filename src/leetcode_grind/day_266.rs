use std::{cell::RefCell, rc::Rc};

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/unique-binary-search-trees-ii/description/
pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    type Node = Option<Rc<RefCell<TreeNode>>>;

    let n = n as usize;
    let mut dp: Vec<Vec<Vec<Node>>> = vec![vec![vec![]; n + 1]; n + 1];

    for i in 1..=n {
        dp[i][i] = vec![Some(Rc::new(RefCell::new(TreeNode {
            val: i as i32,
            left: None,
            right: None,
        })))];
    }

    for num_of_nodes in 2..=n {
        for start in 1..=n - num_of_nodes + 1 {
            let end = start + num_of_nodes - 1;

            for i in start..=end {
                let mut left_subtrees = if i != start {
                    dp[start][i - 1].clone()
                } else {
                    vec![]
                };
                if left_subtrees.is_empty() {
                    left_subtrees.push(None);
                }

                let mut right_subtrees = if i != end {
                    dp[i + 1][end].clone()
                } else {
                    vec![]
                };
                if right_subtrees.is_empty() {
                    right_subtrees.push(None);
                }

                for left in left_subtrees.into_iter() {
                    for right in right_subtrees.clone() {
                        let tree_node = TreeNode {
                            val: i as i32,
                            left: left.clone(),
                            right,
                        };

                        dp[start][end].push(Some(Rc::new(RefCell::new(tree_node))));
                    }
                }
            }
        }
    }

    dp[1][n].clone()
}

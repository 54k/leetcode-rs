use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/check-completeness-of-a-binary-tree/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn bfs_approach(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        use std::collections::VecDeque;
        let mut null_found = false;
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if node.is_none() {
                null_found = true;
            } else {
                if null_found {
                    return false;
                }
                let node = node.unwrap();
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow().right.clone());
            }
        }
        true
    }
    fn dfs_approach(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
            if let Some(r) = root {
                let r = r.borrow();
                count_nodes(r.left.clone()) + count_nodes(r.right.clone()) + 1
            } else {
                0
            }
        }
        fn check_indicies(root: Option<Rc<RefCell<TreeNode>>>, idx: usize, n: usize) -> bool {
            if let Some(r) = root {
                if idx >= n {
                    return false;
                }
                let r = r.borrow();
                check_indicies(r.left.clone(), idx * 2 + 1, n)
                    && check_indicies(r.right.clone(), idx * 2 + 2, n)
            } else {
                true
            }
        }
        check_indicies(root.clone(), 0, count_nodes(root))
    }
    bfs_approach(root)
}

// https://leetcode.com/problems/count-square-submatrices-with-all-ones/description/
// https://leetcode.com/problems/count-square-submatrices-with-all-ones/solutions/441306/java-c-python-dp-solution/
// Explanation
//
// dp[i][j] means the size of biggest square with A[i][j] as bottom-right corner.
// dp[i][j] also means the number of squares with A[i][j] as bottom-right corner.
//
// If A[i][j] == 0, no possible square.
// If A[i][j] == 1,
// we compare the size of square dp[i-1][j-1], dp[i-1][j] and dp[i][j-1].
// min(dp[i-1][j-1], dp[i-1][j], dp[i][j-1]) + 1 is the maximum size of square that we can find.
pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] > 0 && i > 0 && j > 0 {
                matrix[i][j] = 1 + matrix[i - 1][j]
                    .min(matrix[i][j - 1])
                    .min(matrix[i - 1][j - 1]);
            }
            ans += matrix[i][j];
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test349() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
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
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{}", is_complete_tree(root));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
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
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        println!("{}", is_complete_tree(root));
    }

    #[test]
    fn test350() {
        println!(
            "{}",
            count_squares(vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]])
        ); // 15
        println!(
            "{}",
            count_squares(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]])
        ); // 7
    }
}

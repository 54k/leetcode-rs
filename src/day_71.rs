// https://leetcode.com/problems/palindrome-partitioning/solutions/857510/palindrome-partitioning/
pub fn partition(s: String) -> Vec<Vec<String>> {
    fn dfs(
        s: &Vec<char>,
        i: usize,
        cur: &mut Vec<String>,
        dp: &mut Vec<Vec<bool>>,
        res: &mut Vec<Vec<String>>,
    ) {
        if i == s.len() {
            res.push(cur.clone());
            return;
        }
        for j in i..s.len() {
            if s[i] == s[j] && (j - i <= 2 || dp[i + 1][j - 1]) {
                dp[i][j] = true;
                cur.push(s[i..=j].iter().collect());
                dfs(s, j + 1, cur, dp, res);
                cur.pop();
            }
        }
    }
    let mut res = vec![];
    let mut dp = vec![vec![false; s.len()]; s.len()];
    dfs(&s.chars().collect(), 0, &mut vec![], &mut dp, &mut res);
    res
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/invert-binary-tree/description/
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    root.as_ref()?;
    let r = root.clone().unwrap();
    let mut r = r.borrow_mut();
    let t = r.left.clone();
    r.left = r.right.clone();
    r.right = t;
    invert_tree(r.left.clone());
    invert_tree(r.right.clone());
    root
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test171() {
        println!("{:?}", partition("a".to_string()));
        println!("{:?}", partition("aab".to_string()));
    }

    #[test]
    fn test172() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{:?}", invert_tree(tree));
    }
}

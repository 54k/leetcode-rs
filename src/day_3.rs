use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn is_match(s: String, p: String) -> bool {
    fn match_rec(s: String, p: String) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }
        let s = s.chars().collect::<Vec<_>>();
        let p = p.chars().collect::<Vec<_>>();
        let first_match = !s.is_empty() && (s[0] == p[0] || p[0] == '.');

        if p.len() >= 2 && p[1] == '*' {
            return match_rec(s.iter().collect(), p.iter().skip(2).collect()) ||
                first_match && match_rec(s.iter().skip(1).collect(), p.iter().collect());
        }
        return first_match && match_rec(s.iter().skip(1).collect(),
                                        p.iter().skip(1).collect());
    }
    match_rec(s, p)
}

#[allow(dead_code)]
pub fn is_match_dp(s: String, p: String) -> bool {
    fn is_match(s: String, p: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let p = p.chars().collect::<Vec<_>>();

        if p.is_empty() {
            return s.is_empty();
        }

        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        for i in 0..p.len() {
            if p[i] == '*' && dp[0][i - 1] {
                dp[0][i + 1] = true;
            }
        }

        for i in 0..s.len() {
            for j in 0..p.len() {
                if p[j] == '.' || s[i] == p[j] {
                    dp[i + 1][j + 1] = dp[i][j];
                }
                if p[j] == '*' {
                    if p[j - 1] != s[i] && p[j - 1] != '.' {
                        dp[i + 1][j + 1] = dp[i + 1][j - 1];
                    } else {
                        dp[i + 1][j + 1] = dp[i + 1][j] || dp[i][j + 1] || dp[i + 1][j - 1];
                    }
                }
            }
        }

        dp[s.len()][p.len()]
    }
    is_match(s, p)
}

#[allow(dead_code)]
pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::cmp::{min, max};
    fn is_overlap(i1: &[i32], i2: &[i32]) -> bool {
        i1[1] >= i2[0]
    }
    intervals.sort_by(|i1, i2| i1[0].cmp(&i2[0]));

    let mut res = vec![intervals.remove(0)];

    for i2 in &intervals {
        let i1 = &res[res.len() - 1];
        if is_overlap(i1, i2) {
            let to_push = vec![min(i1[0], i2[0]), max(i1[1], i2[1])];
            res.pop();
            res.push(to_push);
        } else {
            res.push(i2.clone());
        }
    }
    res
}

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

#[allow(dead_code)]
pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn count(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn count_left(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                Some(x) => 1 + count_left(&x.borrow().left),
                None => 0
            }
        }

        fn count_right(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                Some(x) => 1 + count_right(&x.borrow().right),
                None => 0
            }
        }

        match root {
            None => 0,
            Some(root) => {
                let left = count_left(&root.borrow().left);
                if count_right(&root.borrow().right) == left {
                    (1 << (left + 1)) - 1
                } else {
                    1 + count(&root.borrow().left) + count(&root.borrow().right)
                }
            }
        }
    }
    count(&root)
}

#[allow(dead_code)]
pub fn count_nodes_loops(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn count(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let mut left = root.borrow().left.clone();
                let mut left_count = 0;
                while let Some(n) = left {
                    left = n.borrow().left.clone();
                    left_count += 1;
                }

                let mut right = root.borrow().right.clone();
                let mut right_count = 0;
                while let Some(n) = right {
                    right = n.borrow().right.clone();
                    right_count += 1;
                }

                if right_count == left_count {
                    (1 << (left_count + 1)) - 1
                } else {
                    1 + count(root.borrow().left.clone()) + count(root.borrow().right.clone())
                }
            }
        }
    }
    count(root)
}

#[allow(dead_code)]
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, mut res: Vec<i32>) -> Vec<i32> {
        if let Some(n) = root {
            res = traverse(&n.borrow().left, res);
            res.push(n.borrow().val);
            res = traverse(&n.borrow().right, res);
        }
        res
    }
    traverse(&root, vec![])
}

#[allow(dead_code)]
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(n) = root {
            res.push(n.borrow().val);
            traverse(&n.borrow().left, res);
            traverse(&n.borrow().right, res);
        }
    }
    let mut vv = vec![];
    traverse(&root, &mut vv);
    vv
}

#[allow(dead_code)]
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(n) = root {
            traverse(&n.borrow().left, res);
            traverse(&n.borrow().right, res);
            res.push(n.borrow().val);
        }
    }
    let mut vv = vec![];
    traverse(&root, &mut vv);
    vv
}

#[allow(dead_code)]
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_bst(root: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(n) = root {
            let n = n.borrow();
            let val = n.val;
            let left = &n.left;
            let right = &n.right;

            if val as i64 <= min || val as i64 >= max {
                return false;
            }

            let is_left_smaller = if let Some(l) = left {
                l.borrow().val < val
            } else {
                true
            };

            let is_right_greater = if let Some(l) = right {
                l.borrow().val > val
            } else {
                true
            };

            return is_left_smaller && is_right_greater &&
                is_bst(left, min, n.val as i64) &&
                is_bst(right, n.val as i64, max);
        }
        true
    }

    return if let Some(n) = &root {
        n.borrow().left.is_none() && n.borrow().right.is_none()
    } else { true }
        || is_bst(&root, i64::MIN, i64::MAX);
}

#[cfg(test)]
mod test {
    use crate::day_3::*;

    #[test]
    fn test_24() {
        println!("{:?}", is_match("abc".to_owned(), "a.*".to_owned()));
    }

    #[test]
    fn test_25() {
        println!("{:?}", is_match_dp("abc".to_owned(), "a.*".to_owned()));
    }

    #[test]
    fn test_26() {
        println!("{:?}", merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]));
        println!("{:?}", merge(vec![vec![1, 4], vec![4, 5]]));
        println!("{:?}", merge(vec![vec![1, 4], vec![0, 0]]));
        println!("{:?}", merge(vec![vec![1, 4], vec![0, 2], vec![3, 5]]));
    }

    #[test]
    fn test_27() {
        println!("{:?}", count_nodes(
            Some(
                Rc::new(
                    RefCell::new(
                        TreeNode {
                            val: 0,
                            left: Some(
                                Rc::new(
                                    RefCell::new(
                                        TreeNode {
                                            val: 2,
                                            left: None,
                                            right: None,
                                        }))),
                            right: None,
                        }
                    )
                )
            )
        ));
    }

    #[test]
    fn test_28() {
        println!("{:?}", count_nodes_loops(
            Some(
                Rc::new(
                    RefCell::new(
                        TreeNode {
                            val: 0,
                            left: Some(
                                Rc::new(
                                    RefCell::new(
                                        TreeNode {
                                            val: 2,
                                            left: None,
                                            right: None,
                                        }))),
                            right: None,
                        }
                    )
                )
            )
        ));
    }

    #[test]
    fn test_29() {
        println!("{:?}", inorder_traversal(
            Some(
                Rc::new(
                    RefCell::new(
                        TreeNode {
                            val: 0,
                            left: Some(
                                Rc::new(
                                    RefCell::new(
                                        TreeNode {
                                            val: 2,
                                            left: None,
                                            right: None,
                                        }))),
                            right: None,
                        }
                    )
                )
            )
        ));
    }

    #[test]
    fn test_30() {
        println!("{:?}", preorder_traversal(
            Some(
                Rc::new(
                    RefCell::new(
                        TreeNode {
                            val: 0,
                            left: Some(
                                Rc::new(
                                    RefCell::new(
                                        TreeNode {
                                            val: 2,
                                            left: None,
                                            right: None,
                                        }))),
                            right: None,
                        }
                    )
                )
            )
        ));
    }

    #[test]
    fn test_31() {
        println!("{:?}", postorder_traversal(
            Some(
                Rc::new(
                    RefCell::new(
                        TreeNode {
                            val: 0,
                            left: Some(
                                Rc::new(
                                    RefCell::new(
                                        TreeNode {
                                            val: 2,
                                            left: None,
                                            right: None,
                                        }))),
                            right: None,
                        }
                    )
                )
            )
        ));
    }

    #[test]
    fn test_32() {
        println!("{:?}", is_valid_bst(
            Some(
                Rc::new(
                    RefCell::new(
                        TreeNode {
                            val: 0,
                            left: None,
                            right: Some(
                                Rc::new(
                                    RefCell::new(
                                        TreeNode {
                                            val: 2,
                                            left: None,
                                            right: None,
                                        }))),
                        }
                    )
                )
            )
        ));
        println!("{:?}", is_valid_bst(
            Some(
                Rc::new(
                    RefCell::new(
                        TreeNode {
                            val: i32::MIN,
                            left: None,
                            right: Some(
                                Rc::new(
                                    RefCell::new(
                                        TreeNode {
                                            val: i32::MAX,
                                            left: None,
                                            right: None,
                                        }))),
                        }
                    )
                )
            )
        ));
    }
}


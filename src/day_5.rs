use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn generate_parenthesis_brute(n: i32) -> Vec<String> {
        let mut res = vec![];
        fn is_valid(s: &str) -> bool {
            let mut stack = 0;
            for c in s.chars() {
                match c {
                    '(' => stack += 1,
                    ')' if stack > 0 => stack -= 1,
                    _ => return false,
                };
            }
            stack == 0
        }
        fn permute(n: usize, buf: &mut String, res: &mut Vec<String>) {
            if buf.len() == n {
                if is_valid(buf) {
                    res.push(buf.to_owned());
                }
            } else {
                buf.push('(');
                permute(n, buf, res);
                buf.pop().unwrap();
                buf.push(')');
                permute(n, buf, res);
                buf.pop().unwrap();
            }
        }
        permute(n as usize * 2, &mut String::new(), &mut res);
        res
    }
    fn generate_parenthesis_backtrack(n: i32) -> Vec<String> {
        let mut res = vec![];
        fn permute(max: usize, buf: &mut String, res: &mut Vec<String>, open: usize, close: usize) {
            if buf.len() == max * 2 {
                res.push(buf.to_owned());
            } else {
                if open < max {
                    buf.push('(');
                    permute(max, buf, res, open + 1, close);
                    buf.pop();
                }
                if close < open {
                    buf.push(')');
                    permute(max, buf, res, open, close + 1);
                    buf.pop();
                }
            }
        }
        permute(n as usize, &mut String::new(), &mut res, 0, 0);
        res
    }
    fn generate_parenthesis_closure_number(n: i32) -> Vec<String> {
        let mut res = vec![];
        if n == 0 {
            res.push("".to_owned());
        } else {
            for c in 0..n {
                for left in generate_parenthesis_closure_number(c) {
                    for right in generate_parenthesis_closure_number(n - 1 - c) {
                        res.push(format!("({}){}", left, right));
                    }
                }
            }
        }
        res
    }
    generate_parenthesis_closure_number(n)
}

// Definition for a binary tree node.
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
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = vec![];

    fn traverse_recursive(
        root: Option<Rc<RefCell<TreeNode>>>,
        res: &mut Vec<Vec<i32>>,
        lvl: usize,
    ) {
        if let Some(node) = root {
            if res.is_empty() || res.len() - 1 < lvl {
                res.push(vec![]);
            }
            let vv = &mut res[lvl];
            let node = node.borrow();
            vv.push(node.val);
            traverse_recursive(node.left.clone(), res, lvl + 1);
            traverse_recursive(node.right.clone(), res, lvl + 1);
        }
    }

    fn traverse_bfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<i32>>) {
        use std::collections::VecDeque;
        let mut q = VecDeque::new();
        if let Some(node) = root {
            q.push_back(node);
            while !q.is_empty() {
                let mut vv = vec![];
                for _ in 0..q.len() {
                    let node = q.pop_front().unwrap();
                    let node = node.borrow();
                    vv.push(node.val);
                    if let Some(node) = node.left.clone() {
                        q.push_back(node);
                    }
                    if let Some(node) = node.right.clone() {
                        q.push_back(node);
                    }
                }
                res.push(vv);
            }
        }
    }

    traverse_bfs(root, &mut res);
    res
}

#[allow(dead_code)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut ins_idx = 1;
    for i in 1..nums.len() {
        if nums[i - 1] != nums[i] {
            nums[ins_idx] = nums[i];
            ins_idx += 1;
        }
    }
    ins_idx as i32
}

#[allow(dead_code)]
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut ins_idx = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[ins_idx] = nums[i];
            ins_idx += 1;
        }
    }
    ins_idx as i32
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
pub fn compute_area(
    ax1: i32,
    ay1: i32,
    ax2: i32,
    ay2: i32,
    bx1: i32,
    by1: i32,
    bx2: i32,
    by2: i32,
) -> i32 {
    fn area(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
        (x2 - x1) * (y2 - y1)
    }
    fn overlap(a1: i32, a2: i32, b1: i32, b2: i32) -> i32 {
        use std::cmp::{max, min};
        let overlap = min(a2, b2) - max(a1, b1);
        max(0, overlap)
    }
    let area_a = area(ax1, ay1, ax2, ay2);
    let area_b = area(bx1, by1, bx2, by2);
    let area_overlap_x = overlap(ax1, ax2, bx1, bx2);
    let area_overlap_y = overlap(ay1, ay2, by1, by2);

    area_a + area_b - (area_overlap_x * area_overlap_y)
}

#[cfg(test)]
mod test {
    use crate::day_5::*;

    #[test]
    fn test39() {
        print!("{:?}", generate_parenthesis(8));
    }

    #[test]
    fn test_40() {
        println!(
            "{:?}",
            level_order(Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))), )
        );
    }

    #[test]
    fn test_41() {
        let mut vv = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        remove_duplicates(&mut vv);
        println!("{:?}", vv)
    }

    #[test]
    fn test_42() {
        let mut vv = vec![0, 1, 2, 2, 3, 0, 4, 2];
        remove_element(&mut vv, 2);
        println!("{:?}", vv)
    }

    #[test]
    fn test_43() {
        println!("{:?}", compute_area(-2, -2, 2, 2, -2, -2, 2, 2));
    }
}

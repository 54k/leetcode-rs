use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/basic-calculator/discuss/62361/Iterative-Java-solution-with-stack
#[allow(dead_code)]
pub fn calculate(s: String) -> i32 {
    let s = s.replace(' ', "");
    let mut stack = vec![];
    let (mut result, mut number, mut sign) = (0, 0, 1);
    for c in s.chars() {
        match c {
            '(' => {
                stack.push(result);
                stack.push(sign);
                sign = 1;
                result = 0;
            }
            ')' => {
                result += sign * number;
                number = 0;
                // is the sign before the parentheses
                result *= stack.pop().unwrap();
                result += stack.pop().unwrap();
            }
            '+' => {
                result += sign * number;
                number = 0;
                sign = 1;
            }
            '-' => {
                result += sign * number;
                number = 0;
                sign = -1;
            }
            _ => number = 10 * number + c as i32 - 48,
        }
    }
    if number != 0 {
        result += sign * number;
    }
    result
}

#[allow(dead_code)]
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut sum = 0;
    for num in nums {
        sum += num;
        res = std::cmp::max(res, sum);
        if sum < 0 {
            sum = 0;
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

type TN = Option<Rc<RefCell<TreeNode>>>;

#[allow(dead_code)]
pub fn is_symmetric(root: TN) -> bool {
    fn is_sym(left: TN, right: TN) -> bool {
        if left.is_some() && right.is_some() {
            let left = left.as_ref().unwrap().borrow();
            let right = right.as_ref().unwrap().borrow();
            return left.val == right.val
                && is_sym(left.right.clone(), right.left.clone())
                && is_sym(left.left.clone(), right.right.clone());
        } else if left.is_none() && right.is_none() {
            return true;
        }
        false
    }
    is_sym(root.clone(), root)
}

#[cfg(test)]
mod test {
    use crate::day_8::*;

    #[test]
    fn test52() {
        // println!("{}", calculate("1 + 1 ".to_owned()));
        // println!("{}", calculate("(1+(4+5+2)-3)+(6+8)".to_owned()));
        println!("{}", calculate("2+(2-1)".to_owned()));
    }

    #[test]
    fn test53() {
        println!("{}", max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
        println!("{}", max_sub_array(vec![-2, 1]));
        println!("{}", max_sub_array(vec![-2, -1]));
    }

    #[test]
    fn test54() {
        println!(
            "{}",
            is_symmetric(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    val: 2,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    val: 2,
                }))),
            }))))
        );
    }
}

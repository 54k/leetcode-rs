// https://leetcode.com/problems/scramble-string/description/
// https://leetcode.com/problems/scramble-string/editorial/
pub fn is_scramble(s1: String, s2: String) -> bool {
    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();
    let n = s1.len();
    let mut dp = vec![vec![vec![false; n]; n]; n + 1];
    for i in 0..n {
        for j in 0..n {
            dp[1][i][j] = s1[i] == s2[j];
        }
    }
    for length in 2..=n {
        for i in 0..n + 1 - length {
            for j in 0..n + 1 - length {
                for new_length in 1..length {
                    let dp1 = dp[new_length][i].clone();
                    let dp2 = dp[length - new_length][i + new_length].clone();
                    dp[length][i][j] |= dp1[j] && dp2[j + new_length];
                    dp[length][i][j] |= dp1[j + length - new_length] && dp2[j];
                }
            }
        }
    }

    dp[n][0][0]
}

// https://leetcode.com/problems/combinations/description/
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    fn rec(n: i32, k: i32, start: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if current.len() == k as usize {
            result.push(current.clone());
            return;
        }
        for i in start..=n {
            current.push(i);
            rec(n, k, i + 1, current, result);
            current.pop();
        }
    }
    let mut result = vec![];
    rec(n, k, 1, &mut vec![], &mut result);
    result
}

// https://leetcode.com/problems/serialize-and-deserialize-bst/description/
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;
struct Codec {}
impl Codec {
    fn new() -> Self {
        Self {}
    }
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn preorder_serialize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
            if let Some(r) = root {
                format!(
                    "{},{},{}",
                    r.as_ref().borrow().val,
                    preorder_serialize(r.as_ref().borrow().left.clone()),
                    preorder_serialize(r.as_ref().borrow().right.clone())
                )
            } else {
                "X".to_string()
            }
        }
        preorder_serialize(root)
    }
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn preorder_deserialize(data: &Vec<&str>, i: &mut i32) -> Option<Rc<RefCell<TreeNode>>> {
            *i += 1;
            if data[*i as usize] == "X" {
                None
            } else {
                Some(Rc::new(RefCell::new(TreeNode {
                    val: data[*i as usize].parse().unwrap(),
                    left: preorder_deserialize(data, i),
                    right: preorder_deserialize(data, i),
                })))
            }
        }
        preorder_deserialize(&data.split(',').collect(), &mut -1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test384() {
        println!("{}", is_scramble("great".to_string(), "rgeat".to_string())); // true
        println!("{}", is_scramble("abcde".to_string(), "caebd".to_string())); // false
        println!("{}", is_scramble("a".to_string(), "a".to_string())); // true
    }

    #[test]
    fn test385() {
        println!("{:?}", combine(4, 2)); // [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
        println!("{:?}", combine(2, 2)); // [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
    }

    #[test]
    fn test386() {
        // it works
    }
}

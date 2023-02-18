use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/wildcard-matching/
pub fn is_match(s: String, p: String) -> bool {
    fn rec(s: &Vec<char>, p: &Vec<char>, i: i32, j: i32) -> bool {
        todo!("try top down impl");
        if i == -1 && j == -1 {
            return true;
        }
        false
    }
    rec(
        &s.chars().collect(),
        &p.chars().collect(),
        s.len() as i32 - 1,
        p.len() as i32 - 1,
    )
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/print-binary-tree/description/
pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
    fn get_height(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
        if let Some(r) = root {
            let r = r.borrow();
            get_height(r.left.clone()).max(get_height(r.right.clone())) + 1
        } else {
            0
        }
    }
    use std::collections::VecDeque;
    let height = get_height(root.clone());
    let width = (2u32.pow(height as u32)) as usize - 1;
    let mut q = VecDeque::new();
    let mut tree = vec![vec!["".to_string(); width]; height];
    q.push_back((root, 0, 0, width - 1));

    while !q.is_empty() {
        let (node, r, lo, hi) = q.pop_front().unwrap();
        if let Some(n) = node {
            let n = n.borrow();
            let c = (lo + hi) / 2;
            tree[r][c] = n.val.to_string();
            if n.left.is_some() {
                q.push_back((n.left.clone(), r + 1, lo, c - 1));
            }
            if n.right.is_some() {
                q.push_back((n.right.clone(), r + 1, c + 1, hi));
            }
        }
    }
    tree
}

// https://leetcode.com/problems/maximum-number-of-balls-in-a-box/
pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    fn get_bucket(mut n: i32) -> usize {
        let mut s = 0;
        while n > 0 {
            s += n % 10;
            n /= 10
        }
        s as usize
    }
    // 100000
    // 99999 == 45
    let mut map = vec![0; 46];
    let mut max = 0;
    for i in low_limit..=high_limit {
        let bucket = &mut map[get_bucket(i)];
        *bucket += 1;
        max = max.max(*bucket);
    }
    max
}

// https://leetcode.com/problems/check-if-all-characters-have-equal-number-of-occurrences/description/
pub fn are_occurrences_equal(s: String) -> bool {
    if s.len() & 1 == 1 {
        false
    } else {
        let mut counts = vec![0; 26];
        let mut max = 0;
        for x in s.chars() {
            let cnt = &mut counts[x as usize - 'a' as usize];
            *cnt += 1;
            max = max.max(*cnt);
        }
        counts.into_iter().filter(|x| *x > 0).all(|x| x == max)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test245() {
        println!(
            "{}",
            is_match("abcabczzzde".to_string(), "*abc???de*".to_string())
        ); // true
        println!("{}", is_match("".to_string(), "*****".to_string())); // true
        println!("{}", is_match("aa".to_string(), "?a*".to_string())); // true
        println!("{}", is_match("aa".to_string(), "*".to_string())); // true
        println!("{}", is_match("".to_string(), "".to_string())); // true
        println!("{}", is_match("aa".to_string(), "a".to_string())); // false
        println!("{}", is_match("cb".to_string(), "?a".to_string())); // false
        println!("{}", is_match("aa".to_string(), "".to_string())); // false
        println!("{}", is_match("".to_string(), "a".to_string())); // false
        println!("{}", is_match("acdcb".to_string(), "a*c?b".to_string())); // false
        println!("{}", is_match("".to_string(), "?".to_string())); // false
    }

    #[test]
    fn test246() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        for r in print_tree(root) {
            println!("{:?}", r);
        }
    }

    #[test]
    fn test247() {
        println!("{}", count_balls(1, 10)); // 2
        println!("{}", count_balls(5, 15)); // 2
        println!("{}", count_balls(19, 28)); // 2
    }

    #[test]
    fn test248() {
        println!("{}", are_occurrences_equal("abacbc".to_string())); // true
        println!("{}", are_occurrences_equal("aaabb".to_string())); // true
    }
}

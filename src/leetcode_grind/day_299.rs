// https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/
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

pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    use std::collections::HashMap;
    let mut lvl = vec![root];
    let mut swaps = 0;

    while lvl.len() > 0 {
        let mut sorted = vec![];
        let mut idx = HashMap::new();

        for (i, node) in lvl.clone().into_iter().enumerate() {
            if let Some(node) = node {
                sorted.push(node.borrow().val);
                idx.insert(node.borrow().val, i);
            }
        }
        sorted.sort();

        let mut visited = vec![false; sorted.len()];
        for i in 0..sorted.len() {
            if visited[i] || idx[&sorted[i]] == i {
                continue;
            }

            let mut cycle_len = 0;
            let mut j = i;
            while !visited[j] {
                visited[j] = true;
                cycle_len += 1;
                j = idx[&sorted[j]];
            }
            if cycle_len > 0 {
                swaps += cycle_len - 1;
            }
        }

        let mut next = vec![];
        for node in lvl {
            if let Some(node) = node {
                let node = node.borrow();

                if node.left.is_some() {
                    next.push(node.left.clone());
                }

                if node.right.is_some() {
                    next.push(node.right.clone());
                }
            }
        }

        lvl = next;
    }

    swaps
}

// https://leetcode.com/problems/closest-prime-numbers-in-range/description/
pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    let max = 1000001.min(right as usize + 1);
    let mut primes = vec![true; max];
    primes[0] = false;
    primes[1] = false;

    let mut i = 2;
    while i < max {
        if primes[i] {
            let mut j = i * i;
            while j < max {
                primes[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let left = left as usize;
    let right = right as usize;

    let mut ans = vec![-1; 2];
    let mut min = usize::MAX;

    let mut i = right;
    while i >= left {
        if primes[i] {
            let mut j = i - 1;
            while j >= left {
                if primes[j] {
                    if i - j <= min {
                        min = i - j;
                        ans[0] = j as i32;
                        ans[1] = i as i32;
                    }
                    i = j + 1;
                    break;
                }
                j -= 1;
            }
        }
        i -= 1;
    }

    ans
}

#[test]
fn test_closest_primes() {
    let ans = closest_primes(10, 19);
    println!("{:?}", ans);
    let ans = closest_primes(12854, 130337);
    println!("{:?}", ans);
}

// https://leetcode.com/problems/binary-search-tree-iterator-ii/
use std::{cell::RefCell, rc::Rc};

type Node = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

struct BSTIterator {
    curr: Node,
    stack: Vec<Node>,
    arr: Vec<i32>,
    idx: i32,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            curr: root,
            stack: vec![],
            arr: vec![],
            idx: -1,
        }
    }

    fn has_next(&self) -> bool {
        self.stack.len() > 0 || self.curr.is_some() || self.idx < self.arr.len() as i32 - 1
    }

    fn next(&mut self) -> i32 {
        self.idx += 1;
        if self.idx == self.arr.len() as i32 {
            while self.curr.clone().is_some() {
                self.stack.push(self.curr.clone());
                self.curr = self.curr.clone().unwrap().borrow().left.clone();
            }
            let curr = self.stack.pop().unwrap();
            self.curr = curr.as_ref().unwrap().borrow().right.clone();

            let curr_val = curr.as_ref().unwrap().borrow().val;
            self.arr.push(curr_val);
        }
        self.arr[self.idx as usize]
    }

    fn has_prev(&self) -> bool {
        self.idx > 0
    }

    fn prev(&mut self) -> i32 {
        self.idx -= 1;
        self.arr[self.idx as usize]
    }
}

// https://leetcode.com/problems/search-a-2d-matrix-ii/description/
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    fn search_rec(
        matrix: &Vec<Vec<i32>>,
        target: i32,
        left: i32,
        up: i32,
        right: i32,
        down: i32,
    ) -> bool {
        if left > right || up > down {
            return false;
        } else if target < matrix[up as usize][left as usize]
            || target > matrix[down as usize][right as usize]
        {
            return false;
        }

        let mid = left + (right - left) / 2;

        let mut row = up;

        while row <= down && matrix[row as usize][mid as usize] <= target {
            if matrix[row as usize][mid as usize] == target {
                return true;
            }
            row += 1;
        }

        search_rec(matrix, target, left, row, mid - 1, down)
            || search_rec(matrix, target, mid + 1, up, right, row - 1)
    }

    search_rec(
        &matrix,
        target,
        0,
        0,
        matrix[0].len() as i32 - 1,
        matrix.len() as i32 - 1,
    )
}

// https://leetcode.com/problems/count-vowel-strings-in-ranges/
pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashSet;
    let vowels = vec!['a', 'e', 'i', 'o', 'u']
        .into_iter()
        .collect::<HashSet<_>>();
    let mut prefix = vec![0];
    for (i, w) in words.iter().enumerate() {
        let w = w.chars().collect::<Vec<_>>();
        prefix.push(
            prefix[i]
                + if vowels.contains(&w[0]) && vowels.contains(&w[w.len() - 1]) {
                    1
                } else {
                    0
                },
        );
    }
    let mut ans = vec![];
    for q in queries {
        ans.push(prefix[q[1] as usize + 1] - prefix[q[0] as usize]);
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test503() {
        println!(
            "{:?}",
            vowel_strings(
                vec![
                    "aba".to_string(),
                    "bcb".to_string(),
                    "ece".to_string(),
                    "aa".to_string(),
                    "e".to_string()
                ],
                vec![vec![0, 2], vec![1, 4], vec![1, 1]]
            )
        ); // [2,3,0]
    }
}

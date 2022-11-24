use std::cell::RefCell;
use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// Maintain two pointers and update one with a delay of n steps.
#[allow(dead_code)]
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    fn rec(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
        match head {
            None => (None, 1),
            Some(mut node) => {
                let (prev, num) = rec(node.next.take(), n);

                if num == n {
                    (prev, num + 1)
                } else {
                    node.next = prev;
                    (Some(node), num + 1)
                }
            }
        }
    }

    rec(head, n).0
}

#[allow(dead_code)]
pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn size(mut head: &mut Option<Box<ListNode>>) -> i32 {
        let mut size = 0;
        while let Some(h) = head {
            head = &mut h.next;
            size += 1;
        }
        size
    }

    let mid = size(&mut head) / 2;
    let mut pre_mid = &mut head;
    for _ in 0..mid - 1 {
        pre_mid = &mut pre_mid.as_mut().unwrap().next;
    }
    let mid_list = pre_mid.as_mut()?.next.take();
    pre_mid.as_mut()?.next = mid_list?.next.take();
    head
}

#[allow(dead_code)]
pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut before_head = ListNode { val: 0, next: None };
    let mut after_head = ListNode { val: 0, next: None };

    let mut before = &mut before_head;
    let mut after = &mut after_head;

    while let Some(mut h) = head {
        let next = h.next.take();
        if h.val < x {
            before.next = Some(h);
            before = before.next.as_mut().unwrap();
        } else {
            after.next = Some(h);
            after = after.next.as_mut().unwrap();
        }
        head = next;
    }

    before.next = after_head.next;

    before_head.next
}

#[allow(dead_code)]
pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
    fn dfs(board: &mut Vec<Vec<char>>, word: &Vec<char>, pos: usize, i: i32, j: i32) -> bool {
        if pos == word.len() {
            return true;
        }
        if i < 0 || i == board.len() as i32 || j < 0 || j == board[i as usize].len() as i32 {
            return false;
        }
        if board[i as usize][j as usize] != word[pos] {
            return false;
        }

        let ch = board[i as usize][j as usize];
        board[i as usize][j as usize] = '.'; // mark visited

        let res = dfs(board, word, pos + 1, i + 1, j)
            || dfs(board, word, pos + 1, i - 1, j)
            || dfs(board, word, pos + 1, i, j + 1)
            || dfs(board, word, pos + 1, i, j - 1);

        board[i as usize][j as usize] = ch; // mark unvisited
        res
    }

    let word = word.chars().collect::<Vec<_>>();
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if dfs(&mut board, &word, 0, i as i32, j as i32) {
                return true;
            }
        }
    }

    false
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

// https://www.geeksforgeeks.org/sorted-linked-list-to-balanced-bst/
// public TreeNode sortedListToBST(ListNode head) {
//     if(null == head){
//     return null;
//     }
//
//     if(null == head.next){
//         return new TreeNode(head.val);
//     }
//
//     ListNode prev = null;
//     ListNode slow = head;
//     ListNode fast = head;
//
//     while(null != fast && null != fast.next){
//         prev = slow;
//         slow = slow.next;
//         fast = fast.next.next;
//     }
//
//     prev.next = null;
//
//     TreeNode root = new TreeNode(slow.val);
//     root.left = sortedListToBST(head);
//     root.right = sortedListToBST(slow.next);
//
//     return root;
// }
#[allow(dead_code)]
pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn inorder(len: i32, list: &mut Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if len == 0 {
            return None;
        }

        let left = inorder(len / 2, list);
        if let Some(h) = list {
            let mut node = TreeNode::new(h.val);
            *list = h.next.take();
            node.left = left;
            let right = inorder(len - len / 2 - 1, list);
            node.right = right;
            Some(Rc::new(RefCell::new(node)))
        } else {
            None
        }
    }

    let mut next = &head;
    let mut len = 0;
    while let Some(node) = next {
        len += 1;
        next = &node.next;
    }

    inorder(len, &mut head)
}

#[allow(dead_code)]
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn o_n(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let mut lo = 0;
        let mut hi = nums.len() as i32 - 1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            match nums[mid as usize].cmp(&target) {
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid - 1,
                std::cmp::Ordering::Equal => {
                    let mut start = mid as i32;
                    let mut end = start;
                    while start >= 0 && nums[start as usize] == target {
                        start -= 1;
                    }
                    while end < nums.len() as i32 && nums[end as usize] == target {
                        end += 1;
                    }
                    return vec![start + 1, end - 1];
                }
            }
        }
        vec![-1, -1]
    }

    fn log_n(nums: Vec<i32>, target: i32) -> Vec<i32> {
        fn search_1(nums: &Vec<i32>, target: i32) -> i32 {
            let mut res = -1;
            let mut lo = 0;
            let mut hi = nums.len() as i32 - 1;
            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                if nums[mid as usize] >= target {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
                if nums[mid as usize] == target {
                    res = mid as i32;
                }
            }
            res
        }

        fn search_2(nums: &Vec<i32>, target: i32) -> i32 {
            let mut res = -1;
            let mut lo = 0;
            let mut hi = nums.len() as i32 - 1;
            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                if nums[mid as usize] <= target {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
                if nums[mid as usize] == target {
                    res = mid as i32;
                }
            }
            res
        }

        vec![search_1(&nums, target), search_2(&nums, target)]
    }

    log_n(nums, target)
}

#[cfg(test)]
mod test {
    use crate::day_12::*;

    #[test]
    fn test69() {
        println!(
            "{:?}",
            remove_nth_from_end(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode {
                                val: 6,
                                next: Some(Box::new(ListNode { val: 7, next: None }))
                            })),
                        })),
                    })),
                })),
                2
            )
        );
    }

    #[test]
    fn test70() {
        println!(
            "{:?}",
            partition(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 2,
                                next: Some(Box::new(ListNode { val: 5, next: None }))
                            })),
                        })),
                    })),
                })),
                3
            )
        );
    }

    #[test]
    fn test71() {
        println!(
            "{:?}",
            delete_middle(Some(Box::new(ListNode { val: 1, next: None })))
        );
    }

    #[test]
    fn test72() {
        println!(
            "{:?}",
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_owned()
            )
        );
        println!(
            "{:?}",
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "SEE".to_owned()
            )
        );
    }

    #[test]
    fn test73() {
        println!(
            "{:?}",
            sorted_list_to_bst(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })))
        );
        println!(
            "{:?}",
            sorted_list_to_bst(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 5, next: None }))
                        })),
                    })),
                })),
            })))
        );
    }

    #[test]
    fn test74() {
        println!("{:?}", search_range(vec![2, 2, 2], 2));
    }
}

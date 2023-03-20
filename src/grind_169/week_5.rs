use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/path-sum-ii/description/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    fn using_dfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            target_sum: i32,
            path: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
        ) {
            if root.is_none() {
                return;
            }
            let r = root.as_ref().unwrap().borrow();
            let val = r.val;
            path.push(val);
            if r.left.is_none() && r.right.is_none() && target_sum - r.val == 0 {
                ans.push(path.clone());
            }
            dfs(r.left.clone(), target_sum - val, path, ans);
            dfs(r.right.clone(), target_sum - val, path, ans);
            path.pop();
        }
        let mut ans = vec![];
        dfs(root, target_sum, &mut vec![], &mut ans);
        ans
    }

    fn using_bfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut res = vec![];
        queue.push_back((root, vec![], target_sum));
        while let Some((Some(r), mut path, remaining_sum)) = queue.pop_front() {
            let val = r.as_ref().borrow().val;
            path.push(val);

            let left = r.as_ref().borrow().left.clone();
            let right = r.as_ref().borrow().right.clone();

            if remaining_sum - val == 0 && left.is_none() && right.is_none() {
                res.push(path.clone());
            }

            if left.is_some() {
                queue.push_back((left, path.clone(), remaining_sum - val));
            }
            if right.is_some() {
                queue.push_back((right, path.clone(), remaining_sum - val));
            }
        }
        res
    }

    using_bfs(root, target_sum)
}

// https://leetcode.com/problems/longest-consecutive-sequence/description/
// https://leetcode.com/problems/longest-consecutive-sequence/editorial/
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    fn using_sort(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut max_streak = 1;
        let mut current_streak = 1;
        nums.sort();
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                continue;
            }
            if nums[i] + 1 == nums[i + 1] {
                current_streak += 1;
                max_streak = max_streak.max(current_streak);
            } else {
                current_streak = 1
            }
        }
        max_streak
    }
    fn using_set(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let set = nums.iter().copied().collect::<HashSet<i32>>();
        let mut longest_streak = 0;
        for num in nums {
            if !set.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_streak = 1;
                while set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_streak += 1;
                }
                longest_streak = longest_streak.max(current_streak);
            }
        }
        longest_streak
    }
    using_sort(nums)
}

// https://leetcode.com/problems/rotate-array/description/
// https://leetcode.com/problems/rotate-array/solutions/54249/3-line-using-reverse/
fn rotate(nums: &mut Vec<i32>, k: i32) {
    fn short(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        nums.reverse();
        nums[0..k].reverse();
        nums[k..].reverse();
    }
    fn using_reverse(nums: &mut Vec<i32>, k: i32) {
        fn reverse(nums: &mut [i32], mut from: i32, mut to: i32) {
            while from < to {
                nums.swap(from as usize, to as usize);
                from += 1;
                to -= 1;
            }
        }
        let len = nums.len() as i32;
        let k = k % len;
        reverse(nums, 0, len - 1);
        reverse(nums, 0, k - 1);
        reverse(nums, k, len - 1);
    }
    using_reverse(nums, k)
}

// https://leetcode.com/problems/odd-even-linked-list/
// https://leetcode.com/problems/odd-even-linked-list/editorial/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut odd_head = None;
    let mut odd_tail = &mut odd_head;
    let mut even_head = None;
    let mut even_tail = &mut even_head;
    let mut i = 0;
    while let Some(mut node) = head.take() {
        head = node.next.take();
        if i % 2 == 0 {
            odd_tail = &mut odd_tail.insert(node).next;
        } else {
            even_tail = &mut even_tail.insert(node).next;
        }
        i += 1;
    }
    if let Some(eh) = even_head {
        let _ = odd_tail.insert(eh);
    }
    odd_head
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_path_sum() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 13,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));
        println!("{:?}", path_sum(root, 22));
    }

    #[test]
    fn test_longest_consecutive() {
        // println!("{}", longest_consecutive(vec![100, 4, 200, 1, 3, 2])); // 4
        println!(
            "{}",
            longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
        ); // 9
    }

    #[test]
    fn test_rotate() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut arr, 3);
        println!("{:?}", arr); // [5,6,7,1,2,3,4]
        let mut arr = vec![-1];
        rotate(&mut arr, 3);
        println!("{:?}", arr); // [-1]
    }

    #[test]
    fn test_odd_even_list() {
        let root = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        println!("{:?}", odd_even_list(root));
    }
}

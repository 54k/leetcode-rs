use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/two-sum/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::*;
    let mut set = HashMap::new();
    for i in 0..nums.len() {
        if set.contains_key(&(target - nums[i])) {
            return vec![*set.get(&(target - nums[i])).unwrap(), i as i32];
        }
        set.insert(nums[i], i as i32);
    }
    vec![]
}

// https://leetcode.com/problems/valid-parentheses/
pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    for ch in s.chars() {
        match ch {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if let Some(p) = stack.pop() {
                    if p != ch {
                        return false;
                    }
                } else {
                    return false;
                };
            }
            _ => {}
        }
    }
    stack.is_empty()
}

// https://leetcode.com/problems/merge-two-sorted-lists/description/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    fn rec(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut n1) = list1.take() {
            if let Some(mut n2) = list2.take() {
                if n1.val <= n2.val {
                    n1.next = rec(n1.next.take(), Some(n2));
                    Some(n1)
                } else {
                    n2.next = rec(Some(n1), n2.next.take());
                    Some(n2)
                }
            } else {
                Some(n1)
            }
        } else {
            list2.take()
        }
    }
    rec(list1, list2)
}

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = i32::MIN;
    let mut min_price = i32::MAX;
    for i in 0..prices.len() {
        if min_price > prices[i] {
            min_price = prices[i]
        }
        max_profit = max_profit.max(prices[i] - min_price);
    }
    max_profit
}

// https://leetcode.com/problems/valid-palindrome/
pub fn is_palindrome(s: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if i < j && !s[i].is_alphanumeric() {
            i += 1;
            continue;
        }
        if i < j && !s[j].is_alphanumeric() {
            j -= 1;
            continue;
        }

        if s[i].to_lowercase().next() != s[j].to_lowercase().next() {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

// https://leetcode.com/problems/invert-binary-tree/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn rec(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root.clone() {
            let mut r = r.borrow_mut();
            let t = r.left.take();
            r.left = r.right.clone();
            r.right = t;
            rec(r.left.clone());
            rec(r.right.clone());
            root
        } else {
            None
        }
    }
    rec(root)
}

// https://leetcode.com/problems/valid-anagram/description/
pub fn is_anagram(s: String, t: String) -> bool {
    let mut hash = vec![0; 26];
    s.chars()
        .map(|x| x as usize - 'a' as usize)
        .for_each(|x| hash[x] += 1);
    t.chars()
        .map(|x| x as usize - 'a' as usize)
        .for_each(|x| hash[x] -= 1);
    hash.into_iter().filter(|x| *x != 0).count() == 0
}

// https://leetcode.com/problems/binary-search/description/
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo = 0i32;
    let mut hi = nums.len() as i32 - 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid as usize] == target {
            return mid as i32;
        } else if nums[mid as usize] < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum() {
        println!("{:?}", two_sum(vec![2, 7, 11, 15], 9)); // [0,1]
        println!("{:?}", two_sum(vec![3, 2, 4], 6)); // [1,2]
        println!("{:?}", two_sum(vec![3, 3], 6)); // [0,1]
    }

    #[test]
    fn test_valid_parentheses() {
        println!("{:?}", is_valid("()".to_string())); // true
        println!("{:?}", is_valid("()[]{}".to_string())); // true
        println!("{:?}", is_valid("(]".to_string())); // false
    }

    #[test]
    fn test_merge_two_lists() {
        println!(
            "{:?}",
            merge_two_lists(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
            )
        ); // [1,1,2,3,4,4]
    }

    #[test]
    fn test_max_profit() {
        println!("{}", max_profit(vec![7, 1, 5, 3, 6, 4])); // 5
        println!("{}", max_profit(vec![7, 6, 4, 3, 1])); // 0
    }

    #[test]
    fn test_is_valid_palindrome() {
        println!(
            "{}",
            is_palindrome("A man, a plan, a canal: Panama".to_string())
        ); // true
        println!("{}", is_palindrome("race a car".to_string())); // false
        println!("{}", is_palindrome(" ".to_string())); // true
        println!("{}", is_palindrome("0P".to_string())); // false
    }

    #[test]
    fn test_invert_tree() {
        println!(
            "{:?}",
            invert_tree(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))))
        );
    }

    #[test]
    fn test_is_anagram() {
        println!(
            "{}",
            is_anagram("anagram".to_string(), "nagaram".to_string())
        ); // true

        println!("{}", is_anagram("rat".to_string(), "car".to_string())); // false
        println!("{}", is_anagram("aa".to_string(), "bb".to_string())); // false
    }

    #[test]
    fn test_search() {
        println!("{}", search(vec![-1, 0, 3, 5, 9, 12], 9)); // 4
        println!("{}", search(vec![-1, 0, 3, 5, 9, 12], 2)); // -1
        println!("{}", search(vec![5], 5)); // 0
        println!("{}", search(vec![5], -5)); // 0
    }
}

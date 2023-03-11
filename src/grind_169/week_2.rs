use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/move-zeroes/
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut j = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(i, j);
            j += 1;
        }
    }
}

// https://leetcode.com/problems/symmetric-tree/
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

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_symmetric(l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if l.is_none() && r.is_none() {
            true
        } else if l.is_some() && r.is_some() {
            let l = l.unwrap();
            let l = l.borrow();
            let r = r.unwrap();
            let r = r.borrow();

            l.val == r.val
                && is_symmetric(l.right.clone(), r.left.clone())
                && is_symmetric(l.left.clone(), r.right.clone())
        } else {
            false
        }
    }
    is_symmetric(root.clone(), root)
}

// https://leetcode.com/problems/missing-number/description/
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut arithmetic_sum = n as i32 * (n as i32 + 1) / 2;
    for x in nums {
        arithmetic_sum -= x;
    }
    arithmetic_sum
}

// https://leetcode.com/problems/palindrome-number/
// https://leetcode.com/problems/palindrome-number/editorial/
pub fn is_palindrome(mut x: i32) -> bool {
    // Special cases:
    // As discussed above, when x < 0, x is not a palindrome.
    // Also if the last digit of the number is 0, in order to be a palindrome,
    // the first digit of the number also needs to be 0.
    // Only 0 satisfy this property.
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut n = 0;
    while x > n {
        n = n * 10 + x % 10;
        x /= 10;
    }
    n == x || n / 10 == x
}

// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn rec(nums: &Vec<i32>, lo: i32, hi: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if lo > hi {
            return None;
        }
        let m = (lo + hi) / 2;
        let mut mid = TreeNode::new(nums[m as usize]);
        mid.left = rec(nums, lo, m - 1);
        mid.right = rec(nums, m + 1, hi);
        Some(Rc::new(RefCell::new(mid)))
    }
    rec(&nums, 0, nums.len() as i32 - 1)
}

// https://leetcode.com/problems/reverse-bits/
pub fn reverse_bits(x: u32) -> u32 {
    // x.reverse_bits() cheat lol
    let mut ans = 0u32;
    for i in 0..=31 {
        let bit_left = x >> i & 1;
        let bit_pos = 31 - i;
        ans |= bit_left << (bit_pos);
    }
    ans
}

// https://leetcode.com/problems/subtree-of-another-tree/description/
pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    fn is_symmetric(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() && sub_root.is_none() {
            true
        } else if root.is_some() && sub_root.is_none() || root.is_none() && sub_root.is_some() {
            false
        } else {
            let r = root.unwrap();
            let r = r.borrow();
            let sr = sub_root.unwrap();
            let sr = sr.borrow();

            r.val == sr.val
                && is_symmetric(r.left.clone(), sr.left.clone())
                && is_symmetric(r.right.clone(), sr.right.clone())
        }
    }

    fn find(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() || sub_root.is_none() {
            false
        } else if root.is_some() && sub_root.is_some() {
            let r = root.clone().unwrap();
            let r = r.borrow();
            if r.val == sub_root.as_ref().unwrap().borrow().val
                && is_symmetric(root, sub_root.clone())
            {
                true
            } else {
                find(r.left.clone(), sub_root.clone()) || find(r.right.clone(), sub_root)
            }
        } else {
            false
        }
    }

    find(root, sub_root)
}

// https://leetcode.com/problems/squares-of-a-sorted-array/description/
// Follow up: Squaring each element and sorting the new array is very trivial,
// could you find an O(n) solution using a different approach?
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    fn approach1(nums: Vec<i32>) -> Vec<i32> {
        let mut l_half = vec![];
        let mut r_half = vec![];
        for n in nums {
            if n < 0 {
                l_half.push(n * n);
            } else {
                r_half.push(n * n);
            }
        }
        let mut ans = vec![];
        let mut i = l_half.len() as i32 - 1;
        let mut j = 0;
        while i >= 0 || j < r_half.len() as i32 {
            if i >= 0 && j < r_half.len() as i32 {
                if l_half[i as usize] < r_half[j as usize] {
                    ans.push(l_half[i as usize]);
                    i -= 1;
                } else {
                    ans.push(r_half[j as usize]);
                    j += 1;
                }
            } else if i >= 0 {
                ans.push(l_half[i as usize]);
                i -= 1;
            } else {
                ans.push(r_half[j as usize]);
                j += 1;
            }
        }
        ans
    }

    fn approach2(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut k = j as i32;
        while k >= 0 {
            if nums[i].abs() > nums[j].abs() {
                ans.push(nums[i] * nums[i]);
                i += 1;
            } else {
                ans.push(nums[j] * nums[j]);
                j -= 1;
            }
            k -= 1;
        }
        ans.reverse();
        ans
    }

    approach2(nums)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut v = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut v);
        println!("{:?}", v); // [1,3,12,0,0]
    }

    #[test]
    fn test_is_symmetric() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{}", is_symmetric(root)); // true
    }

    #[test]
    fn test_missing_number() {
        println!("{}", missing_number(vec![3, 0, 1])); // 2
    }

    #[test]
    fn test_is_palindrome() {
        println!("{}", is_palindrome(121)); // true
        println!("{}", is_palindrome(-121)); // false
        println!("{}", is_palindrome(10)); // false
    }

    #[test]
    fn test_sorted_array_to_bst() {
        println!("{:?}", sorted_array_to_bst(vec![-10, -3, 0, 5, 9]));
    }

    #[test]
    fn test_reverse_bits() {
        println!("{}", reverse_bits(4294967293)); // 3221225471
    }

    #[test]
    fn test_is_subtree() {
        let sub = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: sub.clone(),
        })));

        println!("{}", is_subtree(root, sub)); // true
    }

    #[test]
    fn test_sorted_squares() {
        println!("{:?}", sorted_squares(vec![-4, -1, 0, 3, 10])); // [0,1,9,16,100]
        println!("{:?}", sorted_squares(vec![-7, -3, 2, 3, 11])); // [4,9,9,49,121]
    }
}

use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/minimize-deviation-in-array/description/
// https://leetcode.com/problems/minimize-deviation-in-array/solutions/955262/c-intuitions-and-flip/
pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
    use std::collections::*;
    let mut heap = BinaryHeap::new();
    let mut result = i32::MAX;
    let mut min_n = i32::MAX;
    for i in 0..nums.len() {
        let n = if nums[i] % 2 == 0 {
            nums[i]
        } else {
            nums[i] * 2
        };
        min_n = min_n.min(n);
        heap.push(n);
    }
    while heap.peek().unwrap() % 2 == 0 {
        let &top = heap.peek().unwrap();
        result = result.min(top - min_n);
        let divided = top / 2;
        min_n = min_n.min(divided);
        heap.push(divided);
        heap.pop();
    }
    result.min(heap.peek().unwrap() - min_n)
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/validate-binary-search-tree/description/
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn inorder_recursive(root: Node, prev: &mut Node) -> bool {
        if let Some(n) = root.clone() {
            let n = n.borrow();
            if !inorder_recursive(n.left.clone(), prev) {
                return false;
            }
            if prev.is_some() && prev.clone().unwrap().borrow().val >= n.val {
                return false;
            }
            *prev = root;
            inorder_recursive(n.right.clone(), prev)
        } else {
            true
        }
    }
    fn iterative_inorder(mut root: Node) -> bool {
        let mut prev = None;
        let mut stack = vec![];
        while !stack.is_empty() || root.is_some() {
            while root.is_some() {
                stack.push(root.clone());
                root = root.unwrap().borrow().left.clone();
            }
            root = stack.pop().unwrap();
            // If next element in inorder traversal
            // is smaller than the previous one
            // that's not BST.
            if prev.is_some() && root.clone().unwrap().borrow().val <= prev.unwrap() {
                return false;
            }
            prev = root.clone().map(|x| x.borrow().val);
            root = root.clone().unwrap().borrow().right.clone();
        }
        true
    }
    iterative_inorder(root)
    // inorder_recursive(root, &mut None)
}

// https://leetcode.com/problems/find-mode-in-binary-search-tree/submissions/904059696/
pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    use std::collections::*;
    let mut frequencies = HashMap::<i32, i32>::new();
    fn rec(
        root: Option<Rc<RefCell<TreeNode>>>,
        freq_map: &mut HashMap<i32, i32>,
        max_freq: &mut i32,
    ) {
        if let Some(n) = root.clone() {
            let n = n.borrow();
            let entry = freq_map.entry(n.val).or_insert(0);
            *entry += 1;
            *max_freq = (*max_freq).max(*entry);
            rec(n.left.clone(), freq_map, max_freq);
            rec(n.right.clone(), freq_map, max_freq);
        }
    }
    let mut max_freq = 0;
    rec(root, &mut frequencies, &mut max_freq);
    frequencies
        .into_iter()
        .filter_map(|(k, v)| if v == max_freq { Some(k) } else { None })
        .collect()
}

// https://leetcode.com/problems/sum-of-nodes-with-even-valued-grandparent/description/
// Traverse the tree keeping the parent and the grandparent.
// If the grandparent of the current node is even-valued, add the value of this node to the answer.
pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn post_order_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        type Node = Option<Rc<RefCell<TreeNode>>>;
        fn traverse(root: Node, parent: Node, grandparent: Node) -> i32 {
            if let Some(n) = root.clone() {
                let n = n.borrow();
                let mut result = 0;
                result += traverse(n.left.clone(), root.clone(), parent.clone());
                result += traverse(n.right.clone(), root, parent);
                if let Some(gp) = grandparent {
                    let gp = gp.borrow();
                    if gp.val % 2 == 0 {
                        result += n.val;
                    }
                }
                result
            } else {
                0
            }
        }
        traverse(root, None, None)
    }

    // https://leetcode.com/problems/sum-of-nodes-with-even-valued-grandparent/solutions/477048/java-c-python-1-line-recursive-solution/
    fn one_liner(root: Option<Rc<RefCell<TreeNode>>>, parent: i32, grandparent: i32) -> i32 {
        if root.is_none() {
            0
        } else {
            let root = root.clone().unwrap();
            let root = root.borrow();
            one_liner(root.left.clone(), root.val, parent)
                + one_liner(root.right.clone(), root.val, parent)
                + if grandparent % 2 == 0 { root.val } else { 0 }
        }
    }

    one_liner(root, 1, 1) // 1 is for first iteration
}

// https://leetcode.com/problems/distinct-echo-substrings/description/
// https://leetcode.com/problems/distinct-echo-substrings/solutions/477643/rolling-equality-counter/
pub fn distinct_echo_substrings(text: String) -> i32 {
    fn brute_tle(text: String) -> i32 {
        use std::collections::*;
        let mut set = HashSet::new();
        let text = text.chars().collect::<Vec<_>>();
        for end in 0..text.len() {
            for start in 0..end {
                if (end - start + 1) % 2 == 0 {
                    let half = (end + 1 - start) / 2;
                    let mut is_good = true;
                    for k in start..start + half {
                        if text[k] != text[k + half] {
                            is_good = false;
                            break;
                        }
                    }
                    if is_good {
                        set.insert(&text[start..=end]);
                    }
                }
            }
        }
        set.len() as i32
    }

    fn sliding_window(text: String) -> i32 {
        use std::collections::*;
        let mut set = HashSet::new();
        let text = text.chars().collect::<Vec<_>>();
        for window_size in 1..=text.len() / 2 {
            let mut same_count = 0;
            for i in 0..window_size {
                if text[i] == text[i + window_size] {
                    same_count += 1i32;
                }
            }
            for i in 0..=text.len() - window_size * 2 {
                // move the window forward, tracking the same count of characters
                if same_count == window_size as i32 {
                    set.insert(&text[i..i + window_size]);
                }
                if i == text.len() - window_size * 2 {
                    // last element of the window, nowhere to move
                    break;
                }
                same_count += (text[i + window_size] == text[i + window_size * 2]) as i32
                    - (text[i] == text[i + window_size]) as i32;
            }
        }
        set.len() as i32
    }

    sliding_window(text)
}

// https://leetcode.com/problems/next-greater-element-i/
pub fn next_greater_element(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    fn build_greater_elements_map(nums: &[i32]) -> HashMap<i32, i32> {
        let mut map = HashMap::new();
        let mut stack = vec![];
        for i in (0..nums.len()).rev() {
            while !stack.is_empty() && *stack.last().unwrap() <= nums[i] {
                stack.pop();
            }
            map.insert(
                nums[i],
                if stack.is_empty() {
                    -1
                } else {
                    *stack.last().unwrap()
                },
            );
            stack.push(nums[i]);
        }
        map
    }
    let next_greater_element_map = build_greater_elements_map(&nums2);
    for i in 0..nums1.len() {
        nums1[i] = *next_greater_element_map.get(&nums1[i]).unwrap();
    }
    nums1
}

// https://leetcode.com/problems/next-greater-element-iii/description/
// https://leetcode.com/problems/next-permutation/description/
pub fn next_greater_element_ii(n: i32) -> i32 {
    fn next_permutation(nums: &mut [char]) {
        let mut pivot_idx = -1;
        // 15432 -> pivot idx is 0
        for i in (1..nums.len()).rev() {
            // find first index which is smaller that any element starting from it till the end
            if nums[i - 1] < nums[i] {
                pivot_idx = (i - 1) as i32;
                break;
            }
        }
        if pivot_idx == -1 {
            // all elements in increasing order, e.g. this is the last permutation of array
            // next permutation is reversed arr [4,3,2,1] -> [1,2,3,4]
            nums.reverse();
            return;
        }
        let pivot_idx = pivot_idx as usize;
        for i in (pivot_idx + 1..nums.len()).rev() {
            // find element greater than pivot and swap them
            if nums[i] > nums[pivot_idx] {
                nums.swap(i, pivot_idx);
                let n = nums.len();
                nums[pivot_idx + 1..n].reverse(); // reverse end of array after swap
                break;
            }
        }
    }
    let nums_orig = format!("{}", n).chars().collect::<Vec<_>>();
    let mut nums = nums_orig;
    next_permutation(&mut nums);
    let next_perm = nums
        .into_iter()
        .fold(0, |acc, v| (acc * 10) + v.to_digit(10).unwrap() as i64);
    if next_perm > i32::MAX as i64 || next_perm <= n as i64 {
        -1
    } else {
        next_perm as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test289() {
        println!("{}", minimum_deviation(vec![1, 2, 3, 4])); // 1
        println!("{}", minimum_deviation(vec![4, 1, 5, 20, 3])); // 3
    }

    #[test]
    fn test290() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        println!("{}", is_valid_bst(root)); // true

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: None,
        })));
        println!("{}", is_valid_bst(root)); // false
    }

    #[test]
    fn test291() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        println!("{:?}", find_mode(root)); // [2]
    }

    #[test]
    fn test292() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        println!("{}", sum_even_grandparent(root)); // 3
    }

    #[test]
    fn test293() {
        println!("{}", distinct_echo_substrings("abcabcabc".to_string())); // 3
        println!(
            "{}",
            distinct_echo_substrings("leetcodeleetcode".to_string())
        ); // 2
    }

    #[test]
    fn test294() {
        println!(
            "{:?}",
            next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
        ); // [-1,3,-1]
    }

    #[test]
    fn test295() {
        println!("{}", next_greater_element_ii(12)); // 21
        println!("{}", next_greater_element_ii(21)); // -1
        println!("{}", next_greater_element_ii(1)); // -1
        println!("{}", next_greater_element_ii(101)); // 110
        println!("{}", next_greater_element_ii(230241)); // 230412
    }
}

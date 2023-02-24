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
                if same_count == window_size as i32 {
                    set.insert(&text[i..i + window_size]);
                }
                if i == text.len() - window_size * 2 {
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
}

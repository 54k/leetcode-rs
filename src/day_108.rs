use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/create-maximum-number/description/
// https://leetcode.com/problems/create-maximum-number/solutions/77286/short-python-ruby-c/
// https://leetcode.com/problems/create-maximum-number/solutions/77283/share-my-21ms-java-solution-with-comments/?orderBy=most_relevant
pub fn max_number(mut nums1: Vec<i32>, mut nums2: Vec<i32>, k: i32) -> Vec<i32> {
    fn prepare(nums: &mut Vec<i32>, k: usize) -> Vec<i32> {
        let mut drop = nums.len() - k;
        let mut stack = vec![];
        for n in nums.iter() {
            while !stack.is_empty() && drop > 0 && *stack.last().unwrap() < *n {
                stack.pop();
                drop -= 1;
            }
            stack.push(*n);
        }
        nums[0..k].iter().copied().take(k).collect()
    }
    fn merge(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut a = VecDeque::from(a);
        let mut b = VecDeque::from(b);
        let mut merged = vec![];
        while a.len() + b.len() > 0 {
            let n = if a > b { &mut a } else { &mut b };
            merged.push(n.pop_front().unwrap());
        }
        merged
    }
    let k = k as usize;
    let mut ans = vec![];
    let mut k1 = (k as i32 - nums2.len() as i32).max(0) as usize;
    while k1 <= k.min(nums1.len()) {
        let p1 = prepare(&mut nums1, k1);
        let p2 = prepare(&mut nums2, k - k1);
        println!("{:?} \n{:?} \n ***", p1, p2);
        let merged = merge(p1, p2);
        ans = ans.max(merged);
        k1 += 1;
    }
    ans
}

// https://leetcode.com/problems/maximum-swap/description/
pub fn maximum_swap(num: i32) -> i32 {
    0
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/find-duplicate-subtrees/
pub fn find_duplicate_subtrees(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<String, (Node, i32)>) -> String {
        if let Some(node) = root.clone() {
            let node = node.borrow();
            let left = dfs(node.left.clone(), map);
            let right = dfs(node.right.clone(), map);
            let key = format!("({}){}({})", left, node.val, right);
            map.entry(key.clone()).or_insert((root, 0)).1 += 1;
            key
        } else {
            "".to_string()
        }
    }
    dfs(root, &mut map);
    map.into_iter()
        .filter_map(|(_, (node, cnt))| if cnt > 1 { Some(node) } else { None })
        .collect()
}

// https://leetcode.com/problems/construct-string-from-binary-tree/description/
pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test305() {
        println!(
            "{:?}",
            max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5)
        ); // [9,8,6,5,3]
    }

    #[test]
    fn test306() {
        println!("{}", maximum_swap(2736)); // 7236
        println!("{}", maximum_swap(9973)); // 9973
    }

    #[test]
    fn test307() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        println!("{:?}", find_duplicate_subtrees(root));
    }

    #[test]
    fn test308() {}
}

use std::cell::RefCell;
use std::collections::HashMap;
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

// https://leetcode.com/problems/flood-fill/description/
pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    fn dfs(
        image: &mut Vec<Vec<i32>>,
        visited: &mut Vec<Vec<i32>>,
        pos_and_color: (i32, i32, i32),
        color: i32,
    ) {
        if pos_and_color.0 < 0
            || pos_and_color.1 < 0
            || pos_and_color.0 >= image.len() as i32
            || pos_and_color.1 >= image[0].len() as i32
            || image[pos_and_color.0 as usize][pos_and_color.1 as usize] != pos_and_color.2
            || visited[pos_and_color.0 as usize][pos_and_color.1 as usize] == 1
        {
            return;
        }
        image[pos_and_color.0 as usize][pos_and_color.1 as usize] = color;
        visited[pos_and_color.0 as usize][pos_and_color.1 as usize] = 1;
        for d in DIR {
            dfs(
                image,
                visited,
                (
                    pos_and_color.0 + d.0,
                    pos_and_color.1 + d.1,
                    pos_and_color.2,
                ),
                color,
            );
        }
    }
    let starting_color = image[sr as usize][sc as usize];
    let mut visited = vec![vec![0; image[0].len()]; image.len()];
    dfs(&mut image, &mut visited, (sr, sc, starting_color), color);
    image
}

// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/
// todo revisit for bst
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn rec(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        }
        let r = root.clone().unwrap();
        let r = r.borrow();
        let left = rec(r.left.clone(), p.clone(), q.clone());
        let right = rec(r.right.clone(), p, q);
        if left.is_some() && right.is_some() {
            return root;
        }

        if left.is_some() {
            left
        } else {
            right
        }
    }

    rec(root, p, q)
}

// https://leetcode.com/problems/balanced-binary-tree/description/
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn rec(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            let height_left = rec(r.left.clone());
            let height_right = rec(r.right.clone());
            if height_left < 0 || height_right < 0 || (height_left - height_right).abs() > 1 {
                -1
            } else {
                height_left.max(height_right) + 1
            }
        } else {
            0
        }
    }
    rec(root) > -1
}

// https://leetcode.com/problems/minimum-depth-of-binary-tree/description/
pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn rec(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            let left_height = rec(r.left.clone());
            let right_height = rec(r.right.clone());
            if left_height == 0 || right_height == 0 {
                left_height + right_height + 1
            } else {
                left_height.min(right_height) + 1
            }
        } else {
            0
        }
    }
    rec(root)
}

// https://leetcode.com/problems/implement-queue-using-stacks/description/
struct MyQueue {
    put: Vec<i32>,
    get: Vec<i32>,
}
impl MyQueue {
    fn new() -> Self {
        Self {
            put: vec![],
            get: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.put.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.get.is_empty() {
            while let Some(e) = self.put.pop() {
                self.get.push(e);
            }
        }
        self.get.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.get.is_empty() {
            while let Some(e) = self.put.pop() {
                self.get.push(e);
            }
        }
        *self.get.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.put.is_empty() && self.get.is_empty()
    }
}

// https://leetcode.com/problems/first-bad-version/description/
// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)
struct BadVersion(i32);

impl BadVersion {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut lo = 1;
        let mut hi = n;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if self.isBadVersion(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    pub fn isBadVersion(&self, version: i32) -> bool {
        version >= self.0
    }
}

// https://leetcode.com/problems/ransom-note/description/
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut dict = vec![0; 26];
    for c in magazine.chars() {
        dict[c as usize - 'a' as usize] += 1;
    }
    for c in ransom_note.chars() {
        let d = &mut dict[c as usize - 'a' as usize];
        if *d == 0 {
            return false;
        }
        *d -= 1;
    }
    true
}

// https://leetcode.com/problems/climbing-stairs/
pub fn climb_stairs(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    for _ in 2..=n + 1 {
        let c = a + b;
        a = b;
        b = c;
    }
    a
}

// https://leetcode.com/problems/longest-palindrome/description/
// https://leetcode.com/problems/longest-palindrome/editorial/
pub fn longest_palindrome(s: String) -> i32 {
    use std::collections::*;
    let mut map = HashMap::new();
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    let mut ans = 0;
    for &v in map.values() {
        ans += v / 2 * 2;
        if ans % 2 == 0 && v % 2 == 1 {
            ans += 1;
        }
    }
    ans
}

// https://leetcode.com/problems/reverse-linked-list/
pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut n) = head.take() {
        let next = n.next.take();
        n.next = prev;
        prev = Some(n);
        head = next;
    }
    prev
}

// https://leetcode.com/problems/majority-element/description/
// https://leetcode.com/problems/majority-element/editorial/
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count = 1;
    let mut candidate = nums[0];
    for n in nums.into_iter().skip(1) {
        if count == 0 {
            candidate = n;
        }
        count += if candidate == n { 1 } else { -1 };
    }
    candidate
}

// https://leetcode.com/problems/add-binary/
pub fn add_binary(a: String, b: String) -> String {
    let a = a.chars().rev().collect::<Vec<_>>();
    let b = b.chars().rev().collect::<Vec<_>>();
    let mut ans = String::new();
    let mut carry = 0;
    let mut i = 0;
    let mut j = 0;
    loop {
        let a_d = if i < a.len() {
            a[j] as i32 - '0' as i32
        } else {
            0
        };
        let b_d = if j < b.len() {
            b[j] as i32 - '0' as i32
        } else {
            0
        };

        i += 1;
        j += 1;

        let sum = a_d + b_d + carry;
        ans.push_str(&(sum % 2).to_string());
        carry = sum / 2;

        if i >= a.len() && j >= b.len() {
            if carry > 0 {
                ans.push_str(&carry.to_string());
            }
            break;
        }
    }
    ans.chars().rev().collect()
}

// https://leetcode.com/problems/diameter-of-binary-tree/
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> (i32, i32) {
        if let Some(r) = root {
            let r = r.borrow();
            let left_path = dfs(r.left.clone(), max_diameter);
            let right_path = dfs(r.right.clone(), max_diameter);
            *max_diameter = (*max_diameter)
                .max(left_path.0.max(left_path.1) + 1 + right_path.0.max(right_path.1));
            (
                left_path.0.max(left_path.1) + 1,
                right_path.0.max(right_path.1) + 1,
            )
        } else {
            (0, 0)
        }
    }

    let mut max_diameter = 0;
    dfs(root, &mut max_diameter);
    max_diameter - 1 // num of edges
}

// https://leetcode.com/problems/middle-of-the-linked-list/
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    unsafe {
        let mut slow = Box::into_raw(head.unwrap());
        let mut fast = slow.as_ref();
        while fast.is_some() && fast.unwrap().next.is_some() {
            fast = fast.unwrap().next.as_ref().unwrap().next.as_deref();
            slow = Box::into_raw((*slow).next.take().unwrap());
        }
        Some(Box::from_raw(slow))
    }
}

// https://leetcode.com/problems/maximum-depth-of-binary-tree/
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            dfs(r.left.clone()).max(dfs(r.right.clone())) + 1
        } else {
            0
        }
    }
    dfs(root)
}

// https://leetcode.com/problems/contains-duplicate/description/
pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
    nums.sort();
    for i in 0..nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            return true;
        }
    }
    false
}

// https://leetcode.com/problems/roman-to-integer/
pub fn roman_to_int(s: String) -> i32 {
    use std::collections::*;
    let symbols = vec![
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
    .into_iter()
    .collect::<HashMap<char, i32>>();
    let mut res = 0;
    let s = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    while i < s.len() {
        let cur = symbols[&s[i]];
        let next = if i < s.len() - 1 && symbols[&s[i + 1]] > cur {
            let v = symbols[&s[i + 1]];
            i += 1;
            v
        } else {
            0
        };
        i += 1;
        res += (next - cur).abs();
    }
    res
}

// https://leetcode.com/problems/backspace-string-compare/description/
// https://leetcode.com/problems/backspace-string-compare/editorial/
// Follow up: Can you solve it in O(n) time and O(1) space?
pub fn backspace_compare(s: String, t: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let mut i = s.len() as i32 - 1;
    let mut skip_i = 0;
    let mut j = t.len() as i32 - 1;
    let mut skip_j = 0;

    while i >= 0 || j >= 0 {
        while i >= 0 {
            if s[i as usize] == '#' {
                i -= 1;
                skip_i += 1;
            } else if skip_i > 0 {
                i -= 1;
                skip_i -= 1;
            } else {
                break;
            }
        }

        while j >= 0 {
            if t[j as usize] == '#' {
                j -= 1;
                skip_j += 1;
            } else if skip_j > 0 {
                j -= 1;
                skip_j -= 1;
            } else {
                break;
            }
        }

        if i >= 0 && j >= 0 && s[i as usize] != t[j as usize] {
            return false;
        }

        if (i >= 0) != (j >= 0) {
            // empty vs full
            return false;
        }

        i -= 1;
        j -= 1;
    }

    true
}

// https://leetcode.com/problems/single-number/
pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |acc, v| acc ^ v)
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

    #[test]
    fn test_flood_fill() {
        println!(
            "{:?}",
            flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2)
        ); // [[2,2,2],[2,2,0],[2,0,1]]

        println!(
            "{:?}",
            flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0)
        ); // [[0,0,0],[0,0,0]]

        println!(
            "{:?}",
            flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 1, 0, 2)
        ); // [[2,2,2],[2,2,2]]
    }

    #[test]
    fn test_lowest_common_ancestor() {
        let p = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: p.clone(),
                val: 2,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: q.clone(),
                val: 2,
            }))),
        })));

        println!("{:?}", lowest_common_ancestor(root, p, q)); // return 1 root
    }

    #[test]
    fn test_is_balanced() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))).clone(),
                val: 2,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))).clone(),
                val: 2,
            }))),
        })));

        println!("{:?}", is_balanced(root)); // true
    }

    #[test]
    fn test_min_depth() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))).clone(),
                val: 2,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))).clone(),
                val: 2,
            }))),
        })));

        println!("{:?}", min_depth(root)); // 3
    }

    #[test]
    fn test_my_queue() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        println!("{}", q.peek());
        println!("{}", q.pop());
        println!("{}", q.empty());
    }

    #[test]
    fn test_first_bad_version() {
        let bv = BadVersion(4);
        println!("{}", bv.first_bad_version(5)); // 4
        let bv = BadVersion(1);
        println!("{}", bv.first_bad_version(1)); // 1
        let bv = BadVersion(1702766719);
        println!("{}", bv.first_bad_version(2126753390)); // 1702766719
    }

    #[test]
    fn test_can_construct() {
        println!("{}", can_construct("aa".to_string(), "aab".to_string())); // true
        println!("{}", can_construct("aa".to_string(), "ab".to_string())); // false
        println!("{}", can_construct("a".to_string(), "b".to_string())); // false
    }

    #[test]
    fn test_climb_stairs() {
        println!("{}", climb_stairs(2)); // 2
        println!("{}", climb_stairs(3)); // 3
    }

    #[test]
    fn test_longest_palindrome() {
        println!("{}", longest_palindrome("abccccdd".to_string())); // 7
        println!("{}", longest_palindrome("a".to_string())); // 1
    }

    #[test]
    fn test_reverse_list() {
        println!(
            "{:?}",
            reverse_list(Some(Box::new(ListNode {
                next: Some(Box::new(ListNode {
                    next: Some(Box::new(ListNode { next: None, val: 3 })),
                    val: 2
                })),
                val: 1
            })))
        );
    }

    #[test]
    fn test_majority_element() {
        println!("{}", majority_element(vec![3, 2, 3])); // 3
        println!("{}", majority_element(vec![2, 2, 1, 1, 1, 2, 2])); // 3
    }

    #[test]
    fn test_add_binary() {
        println!("{}", add_binary("11".to_string(), "1".to_string())); // 100
        println!("{}", add_binary("1010".to_string(), "1011".to_string())); // 10101
    }

    #[test]
    fn test_diameter_of_binary_tree() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                val: 2,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                left: None,
                right: None,
                val: 3,
            }))),
        })));

        println!("{}", diameter_of_binary_tree(root)); // 3
    }

    #[test]
    fn test_middle_node() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        println!("{:?}", middle_node(list)); // 2->4
    }

    #[test]
    fn test_max_depth() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                val: 2,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                left: None,
                right: None,
                val: 3,
            }))),
        })));

        println!("{}", max_depth(root)); // 3
    }

    #[test]
    fn test_contains_duplicate() {
        println!("{}", contains_duplicate(vec![1, 2, 3, 1])); // true
        println!("{}", contains_duplicate(vec![1, 2, 3, 4])); // false
        println!("{}", contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2])); // true
    }

    #[test]
    fn test_roman_to_int() {
        println!("{}", roman_to_int("III".to_string())); // 3
        println!("{}", roman_to_int("LVIII".to_string())); // 58
        println!("{}", roman_to_int("MCMXCIV".to_string())); // 1994
    }

    #[test]
    fn test_backspace_compare() {
        println!(
            "{}",
            backspace_compare("ab#c".to_string(), "ad#c".to_string())
        ); // true

        println!(
            "{}",
            backspace_compare("ab##".to_string(), "c#d#".to_string())
        ); // true

        println!("{}", backspace_compare("a#c".to_string(), "b".to_string())); // true

        println!(
            "{}",
            backspace_compare("bbbextm".to_string(), "bbb#extm".to_string())
        ); // false
    }

    #[test]
    fn test_single_number() {
        println!("{}", single_number(vec![2, 2, 1])); // 1
        println!("{}", single_number(vec![4, 1, 2, 1, 2])); // 4
    }
}
